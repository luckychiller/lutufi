# Numerical Stability in Probabilistic Computation

**Document Version 1.0**  
**Status:** Working Draft — Research Phase  
**Author:** Wasswa Lutufi Sebbanja  
**Last Updated:** March 2026  
**License:** Apache 2.0

---

## Table of Contents

1. [The Floating Point Problem](#the-floating-point-problem)
2. [Underflow and Overflow](#underflow-and-overflow)
3. [Log-Space Arithmetic](#log-space-arithmetic)
4. [The Log-Sum-Exp Function](#the-log-sum-exp-function)
5. [Log-Domain Message Passing](#log-domain-message-passing)
6. [Softmax and the Logit Transform](#softmax-and-the-logit-transform)
7. [Sparse Representations](#sparse-representations)
8. [Precision and Roundoff Error](#precision-and-roundoff-error)
9. [MCMC and Numerical Issues](#mcmc-and-numerical-issues)
10. [Matrix Exponentials](#matrix-exponentials)
11. [Linear Algebra for Probabilistic Models](#linear-algebra-for-probabilistic-models)
12. [Gradient Computation](#gradient-computation)
13. [Precision in Variational Inference](#precision-in-variational-inference)
14. [Testing for Numerical Issues](#testing-for-numerical-issues)
15. [Best Practices](#best-practices)
16. [How Lutufi Ensures Numerical Stability](#how-lutufi-ensures-numerical-stability)
17. [Key References](#key-references)

---

## The Floating Point Problem

Computers represent real numbers using finite-precision floating-point arithmetic, typically following the IEEE 754 standard. This representation enables efficient computation but introduces fundamental limitations that profoundly affect probabilistic algorithms.

### IEEE 754 Representation

IEEE 754 double-precision floating-point numbers use 64 bits:
- 1 bit: Sign
- 11 bits: Exponent (biased by 1023)
- 52 bits: Mantissa (significand), with an implicit leading 1

A number is represented as:

$$(-1)^{\text{sign}} \times 2^{\text{exponent} - 1023} \times (1.\text{mantissa})_2$$

**Range of doubles:**
- Smallest positive normal: $2^{-1022} \approx 2.2 \times 10^{-308}$
- Largest finite: $(2 - 2^{-52}) \times 2^{1023} \approx 1.8 \times 10^{308}$
- Machine epsilon ($\epsilon_{mach}$): $2^{-52} \approx 2.2 \times 10^{-16}$

### Machine Epsilon

**Machine epsilon** is the gap between 1 and the next representable floating-point number:

$$\epsilon_{mach} = \min\{ \epsilon > 0 : 1 + \epsilon > 1 \} = 2^{-52}$$

It bounds the relative error of arithmetic operations. For any operation $\circ \in \{+, -, \times, /\}$:

$$\text{fl}(x \circ y) = (x \circ y)(1 + \delta), \quad |\delta| \leq \epsilon_{mach}$$

### Where Probabilistic Computation Fails

Probabilistic computation encounters numerical issues in several scenarios:

1. **Probability products:** Computing $P(x_1, \ldots, x_n) = \prod_{i=1}^n P(x_i)$ for large $n$ causes underflow even when each $P(x_i) \approx 0.5$:
   $$0.5^{1000} = 9.3 \times 10^{-302} \approx \text{underflow threshold}$$

2. **Exponentials:** Computing $\exp(x)$ for moderate $|x|$ causes overflow/underflow:
   - $\exp(1000)$ overflows to infinity
   - $\exp(-1000)$ underflows to zero

3. **Ratios of large/small numbers:** Ratios amplify relative errors

4. **Catastrophic cancellation:** Subtracting nearly equal large numbers loses precision:
   $$1.000000000000001 - 1.000000000000000 = 1.0 \times 10^{-15}$$
   (5 significant digits lost from operands with 16 digits)

5. **Accumulation of roundoff:** Summing many values accumulates error linearly with $n$

---

## Underflow and Overflow

### Why Multiplying Many Probabilities Causes Underflow

Bayesian inference frequently requires computing products of probabilities. For a chain of $n$ binary variables with $P(x_i = 1) = 0.1$:

$$P(x_1 = 1, \ldots, x_n = 1) = 0.1^n$$

- $n = 100$: $10^{-100}$ (representable)
- $n = 308$: $\approx 10^{-308}$ (near underflow)
- $n = 400$: $10^{-400}$ (underflows to 0)

Once a value underflows to zero, it contaminates all downstream computations. A zero probability cannot be recovered, and division by zero may crash the program.

**Real-world example:** Computing the likelihood of a sequence under a hidden Markov model with 1000 time steps. If emission probabilities average 0.01, the total likelihood is $10^{-2000}$, far below the underflow threshold.

### Why Exponentials Cause Overflow

The softmax function and exponential family distributions require computing $\exp(x)$:

- Log-likelihoods of $-1000$ are common for poor models
- $\exp(-(-1000)) = \exp(1000)$ overflows
- $\exp(-1000)$ underflows to 0

In softmax normalization:

$$\text{softmax}(x_i) = \frac{\exp(x_i)}{\sum_j \exp(x_j)}$$

If any $x_j > 709$ (for doubles), $\exp(x_j)$ overflows. If all $x_j < -745$, all exponentials underflow.

### Examples

**Example 1: HMM Likelihood**

Forward algorithm for HMM with $T = 1000$ steps:

$$\alpha_t(j) = \sum_i \alpha_{t-1}(i) a_{ij} b_j(o_t)$$

Each $\alpha_t$ is a probability that decreases exponentially with $t$. By $t = 300$, values underflow.

**Example 2: Bayesian Network Inference**

Computing the joint probability of all variables in a large network:

$$P(X_1, \ldots, X_n) = \prod_{i=1}^n P(X_i \mid \text{Pa}(X_i))$$

For $n = 100$ nodes with average conditional probability 0.5, the joint probability is $0.5^{100} = 7.9 \times 10^{-31}$ (representable). For $n = 1000$, it is $9.3 \times 10^{-302}$ (near underflow). For $n = 2000$, it underflows.

**Example 3: Sampling from High-Dimensional Gaussians**

The unnormalized density of a high-dimensional Gaussian:

$$\exp\left(-\frac{1}{2}(\mathbf{x} - \boldsymbol{\mu})^T \Sigma^{-1} (\mathbf{x} - \boldsymbol{\mu})\right)$$

For $\mathbf{x}$ far from $\boldsymbol{\mu}$ in high dimensions, the exponent is large negative, causing underflow.

---

## Log-Space Arithmetic

Working in log-space transforms multiplicative operations to additive ones, dramatically expanding the dynamic range and preventing underflow/overflow.

### Log Probabilities

Represent probabilities by their logarithms:

$$\tilde{p} = \log p$$

Now $p \in [0, 1]$ becomes $\tilde{p} \in [-\infty, 0]$.

**Representable range:**
- Smallest positive double: $\log(2^{-1074}) \approx -744$
- Largest double: $\log(1.8 \times 10^{308}) \approx 709$

In log-space, we can represent probabilities as small as $e^{-744} \approx 10^{-323}$, vastly extending the usable range.

### Multiplication Becomes Addition

Products become sums in log-space:

$$\log(p \times q) = \log p + \log q = \tilde{p} + \tilde{q}$$

Chains of products:

$$\log\left(\prod_{i=1}^n p_i\right) = \sum_{i=1}^n \log p_i = \sum_{i=1}^n \tilde{p}_i$$

This transforms potentially underflowing products into stable sums.

**Example:** Computing $0.5^{10000}$:
- Direct: Underflows immediately
- Log-space: $10000 \times \log(0.5) = -6931$ (perfectly representable)

### The Log-Sum-Exp Trick for Addition

The challenge in log-space is addition. We need:

$$\log(p + q) = \log(\exp(\tilde{p}) + \exp(\tilde{q}))$$

Direct computation of $\exp(\tilde{p}) + \exp(\tilde{q})$ reintroduces underflow/overflow.

**Log-sum-exp trick:**

$$\log(e^a + e^b) = c + \log(e^{a-c} + e^{b-c})$$

where $c = \max(a, b)$.

Now the largest exponential is $e^0 = 1$, and the other is $\leq 1$, preventing overflow. If $|a - b|$ is large, the smaller term underflows to 0, which is correct (it contributes negligibly to the sum).

---

## The Log-Sum-Exp Function

The log-sum-exp function is the workhorse of numerically stable probabilistic computation.

### Numerical Stability Derivation

For $n$ values $x_1, \ldots, x_n$:

$$\text{LSE}(x_1, \ldots, x_n) = \log\left(\sum_{i=1}^n e^{x_i}\right)$$

**Stable computation:**

$$c = \max_i x_i$$
$$\text{LSE}(x_1, \ldots, x_n) = c + \log\left(\sum_{i=1}^n e^{x_i - c}\right)$$

All exponentials are of non-positive numbers ($x_i - c \leq 0$), preventing overflow. The largest term contributes $e^0 = 1$, ensuring the sum is at least 1 and the log is non-negative.

**Accuracy:** If $x_i - c < -745$, $e^{x_i - c}$ underflows to 0. This is numerically correct: if $x_i$ is more than 745 less than the maximum, it contributes less than $10^{-323}$ to the sum, below machine precision.

### Max-Subtraction Trick

The max-subtraction trick generalizes:

$$\log\sum_i e^{x_i} = x^* + \log\sum_i e^{x_i - x^*}$$

where $x^* = \max_i x_i$.

For two values:

$$\log(e^a + e^b) = \max(a, b) + \log(1 + e^{-|a-b|})$$

The function $\log(1 + e^{-d})$ for $d > 0$ can be approximated:
- $d = 0$: $\log(2) \approx 0.693$
- $d = 10$: $\approx 4.5 \times 10^{-5}$
- $d > 35$: $< 10^{-15}$ (effectively 0)

### Implementation in Pseudocode

```python
def log_sum_exp(x):
    """
    Numerically stable log-sum-exp.
    Input: array of log-values x = [x1, ..., xn]
    Output: log(sum(exp(x)))
    """
    c = max(x)                    # Find maximum
    return c + log(sum(exp(xi - c) for xi in x))

def log_add(a, b):
    """
    Compute log(exp(a) + exp(b)) stably.
    """
    if a == -inf:
        return b
    if b == -inf:
        return a
    c = max(a, b)
    return c + log(exp(a - c) + exp(b - c))
    # Or: c + log1p(exp(-abs(a-b))) if a == c else b + log1p(exp(-abs(a-b)))
```

**Optimization:** For large arrays, use streaming algorithms to avoid allocating the full shifted array.

---

## Log-Domain Message Passing

Belief propagation and other message passing algorithms are implemented in log-domain for numerical stability.

### Sum-Product in Log-Space

The sum-product algorithm computes marginals via messages:

$$m_{ji}(x_i) = \sum_{x_j} \phi_j(x_j) \psi_{ij}(x_i, x_j) \prod_{k \in N(j) \setminus i} m_{kj}(x_j)$$

In log-space:

$$\tilde{m}_{ji}(x_i) = \log\sum_{x_j} \exp\left(\tilde{\phi}_j(x_j) + \tilde{\psi}_{ij}(x_i, x_j) + \sum_{k \in N(j) \setminus i} \tilde{m}_{kj}(x_j)\right)$$

The inner sum is computed in log-space. The outer log-sum-exp handles the marginalization.

### Log-Belief Propagation

**Algorithm:**

```
Log-Belief-Propagation:
1. Initialize: log-messages tilde_m_{ji} = 0 for all edges
2. Repeat until convergence:
   For each edge (j -> i):
     For each value x_i:
       # Compute log-message
       log_terms = []
       For each x_j:
         term = tilde_phi_j(x_j) + tilde_psi_{ij}(x_i, x_j)
         For each neighbor k of j, k != i:
           term += tilde_m_{kj}(x_j)
         log_terms.append(term)
       tilde_m_{ji}(x_i) = log_sum_exp(log_terms)
3. Compute log-beliefs:
   tilde_b_i(x_i) = tilde_phi_i(x_i) + sum_{j in N(i)} tilde_m_{ji}(x_i)
4. Normalize (in log-space):
   b_i(x_i) = exp(tilde_b_i(x_i) - log_sum_exp(tilde_b_i))
```

**Advantages:**
- No underflow in message products
- Extended dynamic range
- Convergence can be monitored in log-space

### Numerical Stability in Junction Tree Algorithm

The junction tree algorithm requires marginalizing potentials over separator variables:

$$\phi_S(x_S) = \sum_{x_{C \setminus S}} \phi_C(x_C)$$

In log-domain:

$$\tilde{\phi}_S(x_S) = \text{LSE}_{x_{C \setminus S}}(\tilde{\phi}_C(x_C))$$

For large cliques, this requires summing over exponentially many configurations. The log-sum-exp operation is applied to each separator configuration.

**Scaling:** Messages are often normalized at each step (subtracting the maximum) to keep values in a reasonable range, though this is optional in log-space.

---

## Softmax and the Logit Transform

### Log-Odds

The **logit** (log-odds) transform maps probabilities to the real line:

$$\text{logit}(p) = \log\frac{p}{1-p} = \log p - \log(1-p)$$

**Range:** $(-\infty, \infty)$ for $p \in (0, 1)$.

The inverse (expit or logistic function):

$$\text{expit}(x) = \frac{1}{1 + e^{-x}} = \frac{e^x}{1 + e^x}$$

**Stability:** For $x > 0$, compute as $1 / (1 + e^{-x})$. For $x < 0$, compute as $e^x / (1 + e^x)$ or $1 - 1 / (1 + e^x)$.

### Logit/Expit Functions

**Computing logit(p) for p near 0 or 1:**

For $p \approx 0$: $\text{logit}(p) \approx \log p$ (since $1-p \approx 1$)

For $p \approx 1$: Let $q = 1-p$, then $\text{logit}(p) = -\text{logit}(q) \approx -\log q$

**Computing expit(x) for large |x|:**

For $x > 0$: $\text{expit}(x) = 1 / (1 + e^{-x})$. As $x \to \infty$, $e^{-x} \to 0$, so $\text{expit}(x) \to 1$.

For $x < 0$: $\text{expit}(x) = e^x / (1 + e^x)$. As $x \to -\infty$, $e^x \to 0$, so $\text{expit}(x) \to 0$.

**Numerical stability:**
```python
def expit(x):
    if x >= 0:
        return 1 / (1 + exp(-x))
    else:
        e = exp(x)
        return e / (1 + e)
```

### Logistic Function Stability Issues

The softmax function:

$$\text{softmax}(x_i) = \frac{e^{x_i}}{\sum_j e^{x_j}}$$

**Stable computation:**

$$\text{softmax}(x_i) = \frac{e^{x_i - c}}{\sum_j e^{x_j - c}}$$

where $c = \max_j x_j$.

**Log-softmax:** Often needed for computing log-probabilities:

$$\log \text{softmax}(x_i) = x_i - c - \log\sum_j e^{x_j - c} = x_i - \text{LSE}(\mathbf{x})$$

---

## Sparse Representations

Sparse representations exploit the fact that many probabilistic structures have few non-zero entries.

### Why CPTs Grow Exponentially

A conditional probability table (CPT) for a node with $k$ parents, each taking $d$ values, has $d^{k+1}$ entries. For binary variables:
- 10 parents: $2^{11} = 2048$ entries
- 20 parents: $2^{21} = 2,097,152$ entries
- 30 parents: $2^{31} \approx 2$ billion entries

This exponential growth makes dense storage impossible for high-degree nodes.

**Real-world observation:** In many Bayesian networks, CPTs are sparse—most entries are zero or near-zero. Deterministic relationships (e.g., $Y = X_1 \land X_2$) create exact zeros. Context-specific independence creates structural zeros.

### Sparse Tensor Formats

**Coordinate format (COO):** Store only non-zero entries as (row, col, value) tuples.
- Memory: $O(\text{nnz})$ where nnz is number of non-zeros
- Random access: $O(\log \text{nnz})$ with sorting, $O(\text{nnz})$ without

**Compressed sparse row (CSR):** Compress row indices, store column indices and values.
- Memory: $O(\text{nnz} + n)$ for $n$ rows
- Row access: $O(1)$
- Column access: $O(\text{nnz})$

**Compressed sparse column (CSC):** Transpose of CSR; efficient column access.

**Dictionary of keys (DOK):** Hash table mapping (i, j) to value.
- Memory: $O(\text{nnz})$
- Random access: $O(1)$ average

### Memory Efficiency

For a CPT with $n$ configurations and $k$ non-zero entries:

| Format | Memory |
|--------|--------|
| Dense | $O(n)$ |
| Sparse (COO) | $O(k)$ (typically $k \ll n$) |

**Example:** A deterministic OR node with 20 binary parents has $2^{21}$ entries but only $2^{20} + 1$ non-zeros (all configurations where at least one parent is 1, plus the all-zero case). Sparse storage saves 50% in this case, and much more for structured determinism.

### Operations on Sparse Tensors

**Marginalization:** Sum over a variable. In sparse format, iterate over non-zero entries and accumulate.

**Conditioning:** Set evidence. Zero out entries inconsistent with evidence (or mark as invalid).

**Multiplication:** Element-wise product of factors. Use hash join or sort-merge on indices.

**Message passing:** When multiplying messages, only non-zero entries contribute. Sparse-sparse multiplication can be faster than dense-dense for very sparse factors.

---

## Precision and Roundoff Error

### Accumulation of Roundoff

Summing $n$ numbers accumulates roundoff error. The naive summation:

$$S = (((x_1 + x_2) + x_3) + \cdots + x_n)$$

has error bound $O(n \epsilon_{mach})$.

For probabilistic computations summing many small probabilities, this can be significant.

### Kahan Summation

**Kahan summation** reduces error accumulation:

```python
def kahan_sum(x):
    s = 0.0
    c = 0.0          # Running compensation
    for xi in x:
        y = xi - c   # Compensate for lost low-order bits
        t = s + y
        c = (t - s) - y   # What was lost
        s = t
    return s
```

**Error bound:** $O(\epsilon_{mach})$ independent of $n$ (under mild conditions).

**When it matters:** When summing many values of similar magnitude, or when high precision is required in probabilistic computations.

### When It Matters for Probabilities

**Log-sum-exp:** The naive implementation:

```python
# Unstable
c = max(x)
return c + log(sum(exp(xi - c) for xi in x))
```

The sum may suffer from roundoff. Use Kahan summation for the sum, or sorted summation (add smallest terms first).

**Posterior normalization:** When computing posterior probabilities:

$$P(H \mid D) = \frac{P(D \mid H) P(H)}{\sum_{H'} P(D \mid H') P(H')}$$

The denominator is a sum that should use stable summation, especially when likelihoods vary widely.

---

## MCMC and Numerical Issues

### Acceptance Ratio Calculation in Log-Space

The Metropolis-Hastings acceptance probability:

$$\alpha = \min\left(1, \frac{P(\theta^*) P(D \mid \theta^*) q(\theta \mid \theta^*)}{P(\theta) P(D \mid \theta) q(\theta^* \mid \theta)}\right)$$

In log-space:

$$\log \alpha = \min(0, \log P(\theta^*) + \log P(D \mid \theta^*) + \log q(\theta \mid \theta^*) - \log P(\theta) - \log P(D \mid \theta) - \log q(\theta^* \mid \theta))$$

**Advantage:** The ratio becomes a difference of log-probabilities, avoiding division of potentially tiny numbers.

### Metropolis-Hastings Log-Probabilities

**Stable acceptance:**
```python
def log_acceptance_prob(log_p_new, log_p_old, log_q_new_given_old, log_q_old_given_new):
    log_alpha = log_p_new + log_q_old_given_new - log_p_old - log_q_new_given_old
    return min(0, log_alpha)

def accept(log_alpha):
    return log(random()) < log_alpha   # Compare logs instead of exponentiating
```

**Underflow-safe comparison:** Instead of computing $\alpha$ and comparing to uniform, compare $\log \alpha$ to $\log(\text{uniform})$.

### Avoiding Numerical Overflow in Likelihood Calculations

**Working with log-likelihoods:** Always compute and store log-likelihoods, not likelihoods.

**Log-likelihood increments:** For sequential models (HMMs, state-space models), compute log-likelihood increments:

$$\ell_t = \log P(D_t \mid D_{1:t-1})$$

and sum:

$$\ell = \sum_t \ell_t$$

**Rescaling:** For very long sequences, periodically rescale (e.g., subtract mean log-likelihood).

---

## Matrix Exponentials

Matrix exponentials arise in continuous-time Markov chains and differential equations.

### Applications to CTMCs

The transition matrix for a CTMC with generator $Q$:

$$P(t) = e^{Qt} = \sum_{k=0}^{\infty} \frac{(Qt)^k}{k!}$$

Direct computation via the series is unstable for large $t$ or large $||Q||$.

### Scaling and Squaring Method

The scaling and squaring method computes $e^A$ stably:

1. **Scale:** Choose $s$ such that $||A/2^s|| < 1$
2. **Padé approximate:** Compute $e^{A/2^s} \approx R_{pq}(A/2^s)$ where $R_{pq}$ is a $(p,q)$ Padé approximant
3. **Square:** $e^A = (e^{A/2^s})^{2^s}$ by repeated squaring

**Padé approximants:** Rational approximations $R_{pq}(x) = N_{pq}(x) / D_{pq}(x)$ where numerator and denominator are polynomials of degrees $p$ and $q$.

**Common choice:** $R_{33}$ or $R_{55}$ provide good accuracy with reasonable cost.

### Padé Approximation

The diagonal Padé approximant for $e^A$:

$$R_{mm}(A) = \frac{\sum_{j=0}^m \frac{(2m-j)! m!}{(2m)! j! (m-j)!} A^j}{\sum_{j=0}^m \frac{(2m-j)! m!}{(2m)! j! (m-j)!} (-A)^j}$$

For $m = 3$:

$$R_{33}(A) = \frac{I + \frac{1}{2}A + \frac{1}{10}A^2 + \frac{1}{120}A^3}{I - \frac{1}{2}A + \frac{1}{10}A^2 - \frac{1}{120}A^3}$$

**Numerical issues:** The denominator can become ill-conditioned. Use matrix division (solving linear systems) rather than explicit inverse.

---

## Linear Algebra for Probabilistic Models

### Solving Linear Systems for Gaussian Models

Gaussian models require solving linear systems $\Sigma \mathbf{x} = \mathbf{b}$ and computing $\mathbf{b}^T \Sigma^{-1} \mathbf{b}$.

**Don't compute the inverse!** Instead:
1. Cholesky decompose: $\Sigma = LL^T$
2. Solve $L\mathbf{y} = \mathbf{b}$ (forward substitution)
3. Solve $L^T\mathbf{x} = \mathbf{y}$ (backward substitution)

**Cost:** $O(n^3)$ for decomposition, $O(n^2)$ for each solve.

**Stability:** Cholesky is stable for positive definite matrices. For nearly singular matrices, use pivoted Cholesky or add regularization.

### Cholesky Decomposition vs Matrix Inversion

**Cholesky:**
- Computes $L$ such that $\Sigma = LL^T$
- Cost: $n^3/3$ flops
- Numerical stability: Excellent for well-conditioned matrices

**Explicit inversion:**
- Cost: $n^3$ flops (3× Cholesky)
- Stability: Worse; errors accumulate
- Memory: Must store full $n \times n$ matrix

**Quadratic forms:** For $\mathbf{x}^T \Sigma^{-1} \mathbf{x}$:
- Inverse: $\mathbf{x}^T \Sigma^{-1} \mathbf{x}$ (requires inverse)
- Cholesky: $\mathbf{y}^T \mathbf{y}$ where $L^T \mathbf{y} = \mathbf{x}$ (no inverse needed)

### Conditioning Issues

The **condition number** of a matrix:

$$\kappa(\Sigma) = \frac{\lambda_{\max}}{\lambda_{\min}}$$

where $\lambda$ are eigenvalues. Large condition numbers indicate ill-conditioning.

**Effects:**
- Small perturbations in $\Sigma$ cause large changes in $\Sigma^{-1}$
- Cholesky may fail or produce inaccurate results
- Linear system solutions are unstable

**Mitigation:**
- Add small diagonal jitter: $\Sigma + \epsilon I$
- Use higher precision arithmetic
- Use iterative refinement
- Use SVD-based pseudo-inverse for rank-deficient cases

---

## Gradient Computation

### Log-Derivative Trick

The **log-derivative trick** (REINFORCE) computes gradients of expectations:

$$\nabla_\theta \mathbb{E}_{x \sim p_\theta}[f(x)] = \mathbb{E}_{x \sim p_\theta}[f(x) \nabla_\theta \log p_\theta(x)]$$

**Numerical stability:** The score function $\nabla_\theta \log p_\theta(x)$ often has better numerical properties than $\nabla_\theta p_\theta(x) / p_\theta(x)$ directly.

**Variance issues:** High variance requires variance reduction techniques (baselines, control variates).

### Numerical Stability in Score Function Estimators

For discrete distributions (e.g., categorical with probabilities $p_i$):

$$\frac{\partial \log p_i}{\partial \theta_j} = \frac{1}{p_i} \frac{\partial p_i}{\partial \theta_j}$$

When $p_i \approx 0$, this becomes large. Use:
- Clipping: bound gradients
- Softmax temperature: sharpen/flatten distribution
- Straight-through estimators: use continuous relaxation

---

## Precision in Variational Inference

### Natural Gradients

Variational inference optimizes the ELBO:

$$\mathcal{L}(\lambda) = \mathbb{E}_{q_\lambda}[\log p(X, Z) - \log q_\lambda(Z)]$$

Standard gradients in Euclidean space can be unstable. **Natural gradients** account for how parameters change the distribution:

$$\tilde{\nabla}_\lambda \mathcal{L} = F(\lambda)^{-1} \nabla_\lambda \mathcal{L}$$

where $F(\lambda)$ is the Fisher information matrix.

**Stability:** Natural gradients are covariant (invariant to parameterization), leading to more stable optimization.

**Computational cost:** Computing $F(\lambda)^{-1}$ is expensive. Use diagonal approximations or amortized methods.

### Why Standard Gradients Fail

Standard gradients in parameter space ignore the geometry of the statistical manifold:
- Different parameterizations give different gradient magnitudes
- Step sizes that work well in one region may diverge in another
- Orthogonality in parameter space $\neq$ independence in distribution space

**Example:** For a Gaussian $q(z) = \mathcal{N}(z; \mu, \sigma^2)$, gradients w.r.t. $(\mu, \sigma)$ vs. $(\mu, \log \sigma)$ behave differently. Natural gradients are invariant to this choice.

### Maintaining Valid Probability Distributions

During optimization, parameters must remain valid (positive variances, valid probabilities, positive definite covariances).

**Softplus transform:** For positive parameters $\sigma > 0$:

$$\sigma = \text{softplus}(\tilde{\sigma}) = \log(1 + e^{\tilde{\sigma}})$$

**Sigmoid transform:** For probabilities $p \in (0, 1)$:

$$p = \text{sigmoid}(\tilde{p}) = \frac{1}{1 + e^{-\tilde{p}}}$$

**Cholesky parameterization:** For covariance matrices, optimize over Cholesky factors $L$ (lower triangular with positive diagonal), reconstruct $\Sigma = LL^T$.

---

## Testing for Numerical Issues

### Unit Tests for Edge Cases

**Test cases:**
1. Very small probabilities ($p < 10^{-300}$)
2. Very large probabilities (nearly 1)
3. Products of many probabilities
4. Ratios of similar numbers (cancellation)
5. Degenerate cases (uniform distributions, zero variance)

**Example test:**
```python
def test_log_sum_exp_extremes():
    # Large range
    x = np.array([-1000, 0, 1000])
    result = log_sum_exp(x)
    expected = 1000 + np.log(1 + np.exp(-1000) + np.exp(-2000))
    assert np.isclose(result, expected)
    
    # All very negative
    x = np.array([-800, -750, -700])
    result = log_sum_exp(x)
    assert not np.isinf(result)
    assert not np.isnan(result)
```

### Property-Based Testing

**Properties to test:**
1. **Normalization:** Probabilities sum to 1 (within tolerance)
2. **Positivity:** All probabilities in $[0, 1]$
3. **Symmetry:** $D_{KL}(p \| p) = 0$, $I(X; X) = H(X)$
4. **Bounds:** Entropy $\leq \log |\mathcal{X}|$, mutual information $\geq 0$
5. **Invariance:** Results invariant to representation (sparse/dense)

**Hypothesis/QuickCheck:** Generate random valid inputs and verify properties hold.

### Monitoring for NaN/Inf Values

**Runtime checks:**
```python
def check_finite(x, name="value"):
    if np.any(np.isnan(x)):
        raise ValueError(f"{name} contains NaN")
    if np.any(np.isinf(x)):
        raise ValueError(f"{name} contains Inf")
    return x
```

**Logging:** Track min/max values of key quantities (log-likelihoods, gradients, parameters) to detect drift toward numerical limits.

---

## Best Practices

### Defensive Coding

1. **Always work in log-space for probabilities:** Never store or multiply raw probabilities for chains longer than ~100.

2. **Use log-sum-exp for all sums of exponentials:** Never compute $\log\sum e^x$ by direct exponentiation.

3. **Check for valid inputs:** Validate that probabilities are in $[0, 1]$, covariances are positive definite.

4. **Handle edge cases explicitly:** Empty sets, zero probabilities, single-element cases.

5. **Use stable implementations of standard functions:** log1p, expm1, hypot, etc.

### Early Detection

1. **Assert finite values:** Check for NaN and Inf at key computation points.

2. **Monitor condition numbers:** Check matrix condition numbers before inversion.

3. **Track value ranges:** Log minimum/maximum values to detect drift.

4. **Unit test extremes:** Test with very large/small inputs.

### Graceful Degradation

1. **Fallback methods:** If Cholesky fails, try adding jitter or using SVD.

2. **Precision upgrade:** If float64 fails, consider arbitrary precision (mpmath) for critical calculations.

3. **Approximate methods:** Use approximations when exact computation is unstable.

4. **User warnings:** Warn users when numerical issues are detected.

---

## How Lutufi Ensures Numerical Stability

### All-Internal Log-Space Operations

Lutufi performs virtually all probabilistic computations in log-space:
- Log-likelihoods, not likelihoods
- Log-posteriors for inference
- Log-messages for belief propagation

**API design:** Functions accept and return log-probabilities by default. Raw probabilities are converted on entry, results converted on exit only when needed.

### Log-Sum-Exp Implementations

Lutufi provides optimized log-sum-exp implementations:
- Single-precision and double-precision versions
- Streaming algorithms for large arrays
- Hardware-accelerated versions (SIMD)

```python
# Lutufi API
from lutufi.numerics import log_sum_exp, log_dot_exp

# Stable log-sum-exp
log_total = log_sum_exp(log_probs)

# Stable matrix multiplication in log-space
log_C = log_dot_exp(log_A, log_B)  # log(exp(log_A) @ exp(log_B))
```

### Sparse Matrix Usage

Lutufi uses sparse representations throughout:
- Sparse CPTs for discrete factors
- Sparse adjacency matrices for network structure
- Sparse message representations in belief propagation

**Memory savings:** Sparse factors use $O(k)$ memory for $k$ non-zeros vs. $O(d^k)$ for dense.

**Computational savings:** Operations on sparse structures skip zero entries.

### Numerical Diagnostics

Lutufi includes diagnostic tools:
- **Condition number monitoring:** Warns when matrices are ill-conditioned
- **Gradient checking:** Compares analytical and numerical gradients
- **Value range tracking:** Monitors for values approaching limits
- **Test suite:** Comprehensive tests for numerical edge cases

**Example output:**
```
WARNING: Factor f_123 has condition number 1e15. Results may be inaccurate.
Consider simplifying the model or adding regularization.
```

---

## Key References

1. Higham, N. J. (2002). *Accuracy and Stability of Numerical Algorithms* (2nd ed.). SIAM. The definitive reference for numerical analysis and floating-point computation.

2. Moler, C., & Van Loan, C. (2003). Nineteen dubious ways to compute the exponential of a matrix, twenty-five years later. *SIAM Review*, 45(1), 3-49. Comprehensive treatment of matrix exponentials.

3. Press, W. H., Teukolsky, S. A., Vetterling, W. T., & Flannery, B. P. (2007). *Numerical Recipes: The Art of Scientific Computing* (3rd ed.). Cambridge University Press. Practical numerical methods including log-sum-exp and sparse matrices.

4. Goldberg, D. (1991). What every computer scientist should know about floating-point arithmetic. *ACM Computing Surveys*, 23(1), 5-48. Accessible introduction to floating-point issues.

5. IEEE Computer Society (2019). IEEE Standard for Floating-Point Arithmetic (IEEE 754-2019). The official standard.

6. Blanchard, P., Higham, N. J., & Mary, T. (2020). A class of fast and accurate summation algorithms. *SIAM Journal on Scientific Computing*, 42(3), A1541-A1557. Advanced summation methods.

7. Al-Mohy, A. H., & Higham, N. J. (2009). A new scaling and squaring algorithm for the matrix exponential. *SIAM Journal on Matrix Analysis and Applications*, 31(3), 970-989. State-of-the-art matrix exponential.

8. Murphy, K. P. (2012). *Machine Learning: A Probabilistic Perspective*. MIT Press. Section on numerical issues in probabilistic ML.

9. Bishop, C. M. (2006). *Pattern Recognition and Machine Learning*. Springer. Appendix on matrix properties and numerical considerations.

10. Gelman, A., Carlin, J. B., Stern, H. S., Dunson, D. B., Vehtari, A., & Rubin, D. B. (2013). *Bayesian Data Analysis* (3rd ed.). CRC Press. Computational considerations for Bayesian inference.

11. Nocedal, J., & Wright, S. J. (2006). *Numerical Optimization* (2nd ed.). Springer. Numerical methods for optimization including gradient computation.

12. Saad, Y. (2003). *Iterative Methods for Sparse Linear Systems* (2nd ed.). SIAM. Sparse matrix methods.

13. Davis, T. A. (2006). *Direct Methods for Sparse Linear Systems*. SIAM. Sparse direct solvers.

14. MacKay, D. J. C. (2003). *Information Theory, Inference, and Learning Algorithms*. Cambridge University Press. Section on arithmetic coding and numerical precision.

15. Kahan, W. (1965). Further remarks on reducing truncation errors. *Communications of the ACM*, 8(1), 40. Original Kahan summation paper.

---

*This document is part of the Lutufi Mathematical Foundations series. For related topics, see `BAYESIAN_NETWORKS.md`, `INFORMATION_THEORY.md`, and `STOCHASTIC_PROCESSES.md`.*
