import math
import numpy as np
import matplotlib.pyplot as plot
import matplotlib.animation as animation

data = np.genfromtxt('./run_on_bb.log', delimiter=';', names=['gx', 'gy', 'px', 'py', 'phi'])

figure, graph = plot.subplots()
graph.grid(True)

p0, = graph.plot( data['gx'], data['gy'], label='goal')
p1, = graph.plot( data['px'], data['py'], label='position')
p2, = graph.plot( data['px'], data['phi'], label='orientation')
graph.legend(handles=[p0, p1, p2])

plot.show()
