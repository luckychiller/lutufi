"""
Learning module for Lutufi.

Provides high-level Python API for parameter and structure learning
from observational data.
"""

from typing import List, Dict, Any, Optional, Union
import pandas as pd
import numpy as np
from ._lutufi import _RustParameterEstimator, _RustStructureLearner
from .models import BayesianNetwork

class ParameterEstimator:
    """Estimator for learning parameters (CPTs) from data."""

    def __init__(self, method: str = "mle", alpha: float = 0.5):
        """
        Initialize the parameter estimator.

        Args:
            method: 'mle' for Maximum Likelihood or 'bayesian' for Bayesian estimation.
            alpha: Pseudocount for Laplace smoothing (MLE) or ESS (Bayesian).
        """
        self._inner = _RustParameterEstimator(method, alpha, 100)

    def fit(self, model: BayesianNetwork, data: pd.DataFrame) -> None:
        """
        Fit the model parameters to the provided data.

        Args:
            model: The Bayesian Network to fit.
            data: A pandas DataFrame where columns match variable names.
        """
        # Convert DataFrame to a list of dicts for Rust FFI
        # In a real implementation, we should use a more efficient zero-copy method
        data_dicts = data.to_dict('records')
        # Ensure all values are strings for the current FFI bridge
        processed_data = []
        for record in data_dicts:
            processed_data.append({str(k): str(v) for k, v in record.items() if pd.notnull(v)})
        
        self._inner.fit(model._model, processed_data)

class StructureLearner:
    """Learner for discovering network structure from data."""

    def __init__(self):
        """Initialize the structure learner."""
        self._inner = _RustStructureLearner()

    def learn(
        self,
        data: pd.DataFrame,
        method: str = "hc",
        score: str = "bic",
        **kwargs
    ) -> BayesianNetwork:
        """
        Learn the DAG structure from data.

        Args:
            data: Observations.
            method: 'hc' (Hill Climbing), 'ges', 'pc', or 'fci'.
            score: 'bic' or 'bdeu'.
            **kwargs: Algorithm-specific options.

        Returns:
            A new BayesianNetwork with the learned structure.
        """
        data_dicts = data.to_dict('records')
        processed_data = []
        for record in data_dicts:
            processed_data.append({str(k): str(v) for k, v in record.items() if pd.notnull(v)})

        rust_bn = self._inner.learn_structure(processed_data, method)
        return BayesianNetwork(_model=rust_bn)

def fit(model: BayesianNetwork, data: pd.DataFrame, method: str = "mle", **kwargs) -> None:
    """Convenience function to fit a model to data."""
    estimator = ParameterEstimator(method=method, **kwargs)
    estimator.fit(model, data)

def learn_structure(data: pd.DataFrame, method: str = "hc", **kwargs) -> BayesianNetwork:
    """Convenience function to learn network structure from data."""
    learner = StructureLearner()
    return learner.learn(data, method=method, **kwargs)
