import json
import glob
import os
import pytest
import numpy as np

def get_ground_truth_cases():
    cases = []
    case_files = glob.glob("tests/ground_truth/*.json")
    for file_path in case_files:
        with open(file_path, "r") as f:
            cases.append(json.load(f))
    return cases

@pytest.mark.parametrize("case", get_ground_truth_cases())
def test_analytical_ground_truth(case):
    """
    Validates Lutufi's inference against analytical ground truth solutions.
    This test will fail (xfail) until inference is implemented.
    """
    if "ALARM_CPTS_PLACEHOLDER" in str(case):
        pytest.skip("Skipping Alarm network as CPTs are not yet implemented")

    # This is where we would normally call Lutufi's inference engine.
    # For now, we simulate a 'NotImplementedError' or similar.
    
    # from lutufi.models import BayesianNetwork
    # from lutufi.inference import ExactInference
    
    # model = BayesianNetwork.from_json(case)
    # infer = ExactInference(model)
    
    for query in case["queries"]:
        # result = infer.query(query["target"], evidence=query["evidence"])
        # np.testing.assert_allclose(result, query["expected"], atol=1e-10)
        pass

    # For now, we fail with a clear message to indicate work remaining
    pytest.xfail("Inference engine not yet implemented. This test serves as the target.")

if __name__ == "__main__":
    pytest.main([__file__])
