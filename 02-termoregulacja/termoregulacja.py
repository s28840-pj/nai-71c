import numpy
import skfuzzy
from skfuzzy import control as control

def createFuzzySys():

    tempInside = control.Antecedent(numpy.arange(0, 41, 1), 'tempInside')
    tempOutside = control.Antecedent(numpy.arange(-10, 40, 1), 'tempOutside')
    humidity = control.Antecedent(numpy.arange(0, 101, 1), 'humidity')

    acPower = control.Consequent(numpy.arange(0, 101, 1), 'acPower')
    heaterPower = control.Consequent(numpy.arange(0, 101, 1), 'heaterPower')

    tempInside['cold'] = skfuzzy.trapmf(tempInside.universe, [0, 0, 10, 18])
    tempInside['comfortable'] = skfuzzy.trimf(tempInside.universe, [17, 22, 27])
    tempInside['hot'] = skfuzzy.trapmf(tempInside.universe, [25, 30, 40, 40])

    tempOutside['cold'] = skfuzzy.trapmf(tempOutside.universe, [-10, -10, 0, 10])
    tempOutside['mild'] = skfuzzy.trimf(tempOutside.universe, [5, 15, 25])
    tempOutside['hot'] = skfuzzy.trapmf(tempOutside.universe, [20, 30, 40, 40])

    humidity['low'] = skfuzzy.trapmf(humidity.universe, [0, 0, 20, 40])
    humidity['medium'] = skfuzzy.trimf(humidity.universe, [30, 50, 70])
    humidity['high'] = skfuzzy.trapmf(humidity.universe, [50, 80, 100, 100])

    acPower['none'] = skfuzzy.trimf(acPower.universe, [0, 0, 0])
    acPower['low'] = skfuzzy.trimf(acPower.universe, [0, 0, 30])
    acPower['medium'] = skfuzzy.trimf(acPower.universe, [20, 50, 80])
    acPower['high'] = skfuzzy.trimf(acPower.universe, [60, 100, 100])
    acPower['full'] = skfuzzy.trimf(acPower.universe, [100, 100, 100])

    heaterPower['none'] = skfuzzy.trimf(heaterPower.universe, [0, 0, 0])
    heaterPower['low'] = skfuzzy.trimf(heaterPower.universe, [0, 0, 30])
    heaterPower['medium'] = skfuzzy.trimf(heaterPower.universe, [20, 50, 80])
    heaterPower['high'] = skfuzzy.trimf(heaterPower.universe, [60, 100, 100])
    heaterPower['full'] = skfuzzy.trimf(heaterPower.universe, [100, 100, 100])

    rules = [
        control.Rule(tempInside['cold'], acPower['none']),
        control.Rule(tempInside['comfortable'] & tempOutside['hot'], acPower['medium']),
        control.Rule(tempInside['hot'] & humidity['low'] , acPower['medium']),
        control.Rule(tempInside['hot'] & humidity['medium'] , acPower['medium']),
        control.Rule(tempInside['hot'] & humidity['high'] , acPower['full']),

        control.Rule(tempInside['cold'], heaterPower['full']),
        control.Rule(tempInside['cold'] & tempOutside['cold'], heaterPower['full']),
        control.Rule(tempInside['cold'] & tempOutside['mild'], heaterPower['medium']), 
        control.Rule(tempInside['cold'] & tempOutside['hot'], heaterPower['medium']), 
        control.Rule(tempInside['comfortable'] & tempOutside['cold'], heaterPower['medium']), 
        control.Rule(tempInside['comfortable'], heaterPower['low']),
        control.Rule(tempInside['hot'], heaterPower['none'])

    ]

    system = control.ControlSystem(rules)
    simulation = control.ControlSystemSimulation(system)

    return simulation, (tempInside, tempOutside, humidity, acPower, heaterPower)

def run_simulation(simulation, tempIn, tempOut, humid):
    simulation.input['tempInside'] = tempIn
    simulation.input['tempOutside'] = tempOut
    simulation.input['humidity'] = humid

    simulation.compute()

    acValue = simulation.output.get('acPower', 0.0)
    heaterValue = simulation.output.get('heaterPower', 0.0)

    return {
        "acPower": round(acValue, 2),
        "heaterPower": round(heaterValue, 2)
    }

if __name__ == "__main__":
    simulation, _ = createFuzzySys()

    result = run_simulation(simulation, 15, 25, 60)

    print(f"AC: {result['acPower']} %")
    print(f"Heater: {result['heaterPower']} %")
