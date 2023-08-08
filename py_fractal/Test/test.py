import numpy as np
import matplotlib.pyplot as plt
from py_fractal import gen_julia_fractal
print("Generating Fractal")
fractal = gen_julia_fractal(20000, 10000, 2, -0.74543+0.11301j, 1000)
print("Fractal Generated")
plt.imshow(fractal)
plt.show()
print("Done")