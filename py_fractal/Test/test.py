import numpy as np
import matplotlib.pyplot as plt
from py_fractal import gen_julia_fractal

fractal = gen_julia_fractal(1000, 2000, 2, -0.74543+0.11301j, 1000)
plt.imshow(fractal)
plt.show()