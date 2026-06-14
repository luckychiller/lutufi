"""
Atomic editing and the Lutufi exception hierarchy.

This example demonstrates:

  - model.edit(): an atomic context manager for modifying a built model.
    If the block raises or leaves the model invalid, all changes are
    rolled back automatically.
  - Catching specific LutufiError subclasses (e.g. LutufiCyclicGraphError,
    LutufiValidationError) instead of bare ValueError/Exception.

All Lutufi*Error classes subclass ValueError, so existing
`except ValueError` code keeps working even as you adopt the more
specific subclasses.
"""

import lutufi
from lutufi import LutufiCyclicGraphError


def build_base_model() -> lutufi.BayesianNetwork:
    return (
        lutufi.BayesianNetwork.builder()
        .add_variable("A", domain=["0", "1"])
        .add_variable("B", domain=["0", "1"])
        .set_cpd("A", [0.5, 0.5])
        .set_cpd("B", [0.5, 0.5])
        .build()
    )


def main():
    # --- 1. Successful edit ---
    bn = build_base_model()
    print("Before edit:", bn.edges())

    with bn.edit() as m:
        m._model.add_edge("A", "B")
        m._model.set_cpd("B", [[0.9, 0.1], [0.1, 0.9]])

    print("After successful edit:", bn.edges())
    print("is_valid:", bn.is_valid())

    # --- 2. Edit that fails gets rolled back ---
    bn2 = build_base_model()
    try:
        with bn2.edit() as m:
            # Adding the edge succeeds, but giving B a CPT with the wrong
            # shape for its new parent set raises immediately. edit() rolls
            # back the whole block, including the add_edge below.
            m._model.add_edge("A", "B")
            m._model.set_cpd("B", [[0.9], [0.1]])  # wrong shape: 1 parent config given, 2 needed
    except ValueError as e:
        # CPT-shape errors are raised as plain ValueError (a LutufiError
        # subclass would also be caught here, since they all derive from
        # ValueError).
        print("\nEdit rolled back due to error:")
        print(f"  {e}".splitlines()[0])

    print("Edges after rollback:", bn2.edges())  # unchanged: []

    # --- 3. Catching a cyclic-graph error ---
    bn3 = (
        lutufi.BayesianNetwork.builder()
        .add_variable("X", domain=["0", "1"])
        .add_variable("Y", domain=["0", "1"])
        .set_cpd("X", [0.5, 0.5])
        .set_cpd("Y", [0.5, 0.5])
    )
    bn3.add_edge("X", "Y")
    try:
        bn3.add_edge("Y", "X")  # would create a 2-cycle
    except LutufiCyclicGraphError as e:
        print("\nCaught LutufiCyclicGraphError:")
        print(f"  from_node={e.from_node!r}, to_node={e.to_node!r}")
        # LutufiCyclicGraphError (and every Lutufi*Error) is also a
        # ValueError, so this still works:
        assert isinstance(e, ValueError)


if __name__ == "__main__":
    main()
