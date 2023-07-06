# Kolmogorov Generator
## Introduction
`kolmogorov-generator` is a python package for generating random bitstrings sampled according to the universal prior. The universal prior is the output of a turing machine sampled randomly following the development of kolmogorov complexity.

## Installation
* Make sure you have a working rust installation - run rustc --version to check.
  * If you don't have rust installed, follow the instructions at https://www.rust-lang.org/tools/install to install it (a quick and painless process).
* Run `pip install git+https://github.com/KanHarI/kolmogorov-generator` to install the package.

## Usage
```python
import kolmogorov_generator as kg
import os

TAPE_SIZE = 5
GAMMA = 0.6
MAX_STATES = 16
MAX_STEPS = 100
FILTER_UNIFORM_OUTPUTS = True

seed = os.urandom(32)

bitstrings_generator = kg.KolmogorovGen(TAPE_SIZE, GAMMA, MAX_STATES, MAX_STEPS, FILTER_UNIFORM_OUTPUTS, seed)
random_bitstring = bitstrings_generator.generate()
print(random_bitstring)
# array([False, False,  True,  True,  True])
```

## Parameters 
* `tape_size` - The size of the tape used by the turing machine. Size of the output array.
* `gamma` - Decay rate for the number of states in the turing machine. Choosing 0.7 will lead in 30% of the times to a machine with 2 states, in 21% of the times to a machine with 3 states, etc. The default, theoretical kolmogorov complexity requires gamma=0.5, but for practical purposes higher values can be useful.
* `max_states` - The maximum number of states in the turing machine.
* `max_steps` - The maximum number of steps the turing machine can run.
* `filter_uniform_outputs` - If True, the generator will filter out bitstrings that are uniform. Uniform outputs (all 0's, all 1's) are uninteresting for many applications.
* `seed` - A seed for the random number generator. If not provided, a random seed will be used.

Returns a numpy array of booleans.