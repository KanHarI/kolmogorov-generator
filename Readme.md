# Kolmogorov Generator
## Introduction
`kolmogorov-generator` is a python package for generating random bitstrings sampled according to the universal prior. The universal prior is the output of a randomly sampled turing machine. For a general introduction, see [Solomonoff's theory of inductive inference](https://en.wikipedia.org/wiki/Solomonoff%27s_theory_of_inductive_inference).

## Installation
* Make sure you have a working rust installation - run `rustc --version` to check.
  * If you don't have rust installed, follow the instructions at https://www.rust-lang.org/tools/install to install it (a quick and painless process).
* Run `pip install git+https://github.com/KanHarI/kolmogorov-generator` to install the package.

## Usage
```python
import kolmogorov_generator as kg
import os
TAPE_SIZE = 64
GAMMA = 0.8
MAX_STATES = 16
MAX_STEPS = 256
FILTER_UNIFORM_OUTPUTS = True

seed = os.urandom(32)

bitstrings_generator = kg.KolmogorovGen(TAPE_SIZE, GAMMA, MAX_STATES, MAX_STEPS, FILTER_UNIFORM_OUTPUTS, seed)
random_bitstring = bitstrings_generator.generate()
print(random_bitstring)
```

## Parameters 

| Parameter | Description                                                                                                                                                                                                           | Example          |
| --- |-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------|
| `tape_size` | The size of the tape used by the Turing machine. Defines the size of the output array.                                                                                                                                | `64`             |
| `gamma` | Decay rate for the number of states in the Turing machine. The theoretical derivation uses 0.5. Choosing higher values makes larger turing machines more likely. Higher values might be useful for some applications. | `0.8`            |
| `max_states` | The maximum number of states in the Turing machine.                                                                                                                                                                   | `16`             |
| `max_steps` | The maximum number of steps the Turing machine can run. Higher values are needed for higher quality outputs.                                                                                                          | `256`            |
| `filter_uniform_outputs` | If set to `True`, the generator will filter out bitstrings that are uniform (all 0's or all 1's).                                                                                                                     | `True`           |
| `seed` | A seed for the random number generator. If not provided, a random seed will be used.                                                                                                                                  | `os.urandom(32)` |

## Output
The generator returns a numpy array of boolean values representing the generated bitstring.

## Sample
```text
0000000000000000000000000000010100000000000001010100000000000000
0000000100000100000100000100000100000100000100000100000100000000
1111111111111111111111111111111111111111111111111111111111111110
0101010101010101010101010101010101010101010101010101010101010101
1101101101101101101101101010100101010101010110110110110110110110
0000000000000000000000000000000001000000000000000000000000000000
0101010001100001101111100011000110001100011100011000110001001001
0000000000000000000000000000000010000000000000000000000000000000
1011011011011011011011011011011010010010010010101101101101101101
0000000000000000000000000100000000000000000000000000000000000000
1010101010101010101010101010101000001010101010101010101010101010
0000000000000000000000000000000010000000000000000000000000000000
1111111111111111111111111111101111111111111111111111111111111111
0000000000000000000000000000000000010000000000000000000000000000
0000000000000000000000000000000010000000000000000000000000000000
0000000000000000000000000000000010000000000000000000000000000000
0000000000000000000000000011011001000000000000000000000000000000
```

## Other generators

```python
import kolmogorov_generator
import os

OUTER_TAPE_SIZE = 64
INNER_TAPE_SIZE = 64
GAMMA = 0.8
MAX_STATES = 16
MAX_STEPS = 256
FILTER_UNIFORM_OUTPUTS = True

seed = os.urandom(32)

bitstrings_generator = kolmogorov_generator.KolmogorovInnerStateGen(
  OUTER_TAPE_SIZE,
  INNER_TAPE_SIZE,
  GAMMA,
  MAX_STATES,
  MAX_STEPS,
  FILTER_UNIFORM_OUTPUTS,
  seed
)
print(bitstrings_generator.generate())
```
Sample usage:
```python
import kolmogorov_generator as kg
import time

g = kg.KolmogorovInnerStateGen(64, 64, 0.8, 16, 256, True)
while True:
  print(''.join(['1' if x else '0' for x in g.generate()]))
  time.sleep(2)

```
Output:
```text
1111111111111111111111100010000000001111111111111111111111111111
1111111111111111111111111111111111111111111101111111111111111111
1111111111111111111111111111111111111111111111111111011111111111
1111111111111110000000000000000000000000000000001111111111111111
0011111111111111111010000000000000000000000000000000000000000000
1111110111011111101111110110110111111111111111111111111111111111
```
