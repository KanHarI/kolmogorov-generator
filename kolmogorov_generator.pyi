import numpy as np
import numpy.typing as npt

class KolmogorovGen:
    def __init__(
        self,
        tape_size: int,
        states_discount_rate: float,
        max_states: int,
        max_steps: int,
        filter_uniform_outputs: bool,
        seed: bytes,
    ): ...
    def generate(self) -> npt.NDArray[np.bool]: ...
