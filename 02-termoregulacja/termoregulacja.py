"""
Projekt: System sterowania klimatyzacją i ogrzewaniem z użyciem logiki rozmytej.

Opis:
System przyjmuje trzy dane wejściowe:
 - temperaturę wewnętrzną,
 - temperaturę zewnętrzną,
 - wilgotność.

Oraz generuje dwa wyjścia:
 - moc klimatyzacji (0-100%),
 - moc ogrzewania (0-100%).


Wykorzystano logikę rozmytą do określenia płynnych wartości mocy
na podstawie zdefiniowanych reguł i funkcji przynależności.

Biblioteka: scikit-fuzzy (https://github.com/scikit-fuzzy/scikit-fuzzy)

Zakresy dopuszczalnych danych wejściowych:
-----------------------------------------
- Temperatura wewnętrzna:   0 - 40 °C
- Temperatura zewnętrzna:  -10 - 40 °C
- Wilgotność:               0 - 100 %


Zależności:
-------------
- Python 3.12+
- numpy
- scikit-fuzzy
- scipy
- packaging
- pdoc

Instalacja i uruchomienie:
    python -m venv fuzzy_env
    fuzzy_env\Scripts\activate
    pip install scikit-fuzzy numpy scipy packaging pdoc
    python .\termoregulacja.py

Generowanie dokumentacji:
    pdoc -d markdown .\termoregulacja.py

"""

import numpy
import skfuzzy
from skfuzzy import control as control
import csv

def createFuzzySys():

    """
    Tworzy i konfiguruje system logiki rozmytej do sterowania klimatyzacją i ogrzewaniem

	Zwraca:
		tuple: (simulation, (tempInside, tempOutside, humidity, acPower, heaterPower))
			simulation - obiekt ControlSystemSimulation gotowy do podstawienia wejść i obliczeń,
			(tempInside, tempOutside, humidity, acPower, heaterPower) - zawiera obiekty Antecedent/Consequent użyte przy budowie systemu.
	"""

    # Zmienne wejściowe
    tempInside = control.Antecedent(numpy.arange(0, 41, 1), 'tempInside')
    tempOutside = control.Antecedent(numpy.arange(-10, 40, 1), 'tempOutside')
    humidity = control.Antecedent(numpy.arange(0, 101, 1), 'humidity')

    # Zmienne wyjściowe
    acPower = control.Consequent(numpy.arange(0, 101, 1), 'acPower')
    heaterPower = control.Consequent(numpy.arange(0, 101, 1), 'heaterPower')

    # Definicje funkcji przynależności
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

    # Reguły logiki rozmytej
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

def runSimulation(simulation, tempIn, tempOut, humid):

    """
    Funkcja wykonuje pojedyńczą symulację systemu logiki rozmytej

	Parametry:
		simulation (ControlSystemSimulation): przygotowany obiekt symulacji.
		tempIn (float): temperatura wewnętrzna [C].
		tempOut (float): temperatura zewnętrzna [C].
		humid (float): wilgotność [%].

	Zwraca:
		dict: {'acPower': float, 'heaterPower': float} — wyniki sterowania zaokrąglone do 2 miejsc.
	"""

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

def generateCsv(simulation, selection, tempInput, outputFile="grid.csv"):

    """
    Funkcja generuje wyniki dla całego przekroju zakresów systemu i zapisuje do pliku CSV

	Parametry:
		simulation (ControlSystemSimulation): przygotowany obiekt symulacji.
		selection (str): wybór wersji generowania wyników
		tempInput (float): stała wartość temperatury [C]
		outputFile (str): nazwa pliku CSV (default = "grid.csv")

	"""
        
    insideRange = range(0,41)
    outsideRange = range(-10,41)
    humidityRange = range(0, 101)

    result = []

    result.append(["tempInside", "tempOutside", "humidity", "acPower", "heaterPower"])
    
    if selection == "1":
        for tIn in insideRange:
            for hum in humidityRange:
                simulation.input['tempInside'] = tIn
                simulation.input['tempOutside'] = tempInput
                simulation.input['humidity'] = hum

                simulation.compute()

                ac = simulation.output.get('acPower', 0.0)
                heater = simulation.output.get('heaterPower', 0.0)

                result.append([tIn, tempInput, hum, round(ac, 2), round(heater, 2)])

    if selection == "2":
        for tOut in outsideRange:
            for hum in humidityRange:
                simulation.input['tempInside'] = tempInput
                simulation.input['tempOutside'] = tOut
                simulation.input['humidity'] = hum

                simulation.compute()

                ac = simulation.output.get('acPower', 0.0)
                heater = simulation.output.get('heaterPower', 0.0)

                result.append([tempInput, tOut, hum, round(ac, 2), round(heater, 2)])
        
    with open(outputFile, mode='w', newline='', encoding='utf-8') as f:
        writer = csv.writer(f)
        writer.writerows(result)

def getUserInput(prompt, minValue, maxValue):

    """
    Funkcja pobiera od użytkownika dane sprawdzając zadane wartości min, max

	Parametry:
		prompt (str): tekst wyświetlany użytkownikowi.
		minValue (float): minimalna akceptowana wartość.
		maxValue (float): maksymalna akceptowana wartość.

	Zwraca:
		value (float): poprawna wartość w zadanym zakresie.
	"""

    while True:
        try:
            value = float(input(prompt))
            if value < minValue or value > maxValue:
                print(f"Wartość spoza zakresu: ({minValue}/{maxValue}). Spróbuj ponownie.")
            else:
                return value
        except ValueError:
            print("Nieprawidłowa wartość. Wpisz liczbę.")

if __name__ == "__main__":
    simulation, _ = createFuzzySys()

    print("Menu")
    print("1. Uruchom jedną symulację")
    print("2. Wygeneruj CSV z wszystkimi wynikami")

    choice = input("Wybierz opcję: ").strip()

    if choice == "1":
        tempIn = getUserInput("Temperatura wewnętrzna [C] (0/40): ", 0, 40)
        tempOut = getUserInput("Temperatura zewnętrzna [C] (-10/40): ", -10, 40)
        humid = getUserInput("Wilgotność [%] (0-100): ", 0, 100)
        result = runSimulation(simulation, tempIn, tempOut, humid)

        print("Wyniki symulacji:")
        print(f"Klimatyzacja: {result['acPower']} %")
        print(f"Ogrzewanie: {result['heaterPower']} %")
    
    elif choice == "2":
        print("Menu")
        print("1. Wygeneruj dla temperatury zewnętrznej")
        print("2. Wygeneruj dla temperatury wewnętrznej")
        choice = input("Wybierz opcję: ").strip()
        
        if choice == "1":
            temp = getUserInput("Podaj temperaturę zewnętrzną [C] (-10/40): ", -10, 40)
            fileName = input("Podaj nazwę pliku (np. grid.csv): ").strip() or "grid.csv"
            generateCsv(simulation, choice, temp, fileName)
        elif choice == "2":
            temp = getUserInput("Podaj temperaturę wewnętrzną [C] (0/40): ", 0, 40)
            fileName = input("Podaj nazwę pliku (np. grid.csv): ").strip() or "grid.csv"
            generateCsv(simulation, choice, temp, fileName)

    else:
        print("Błędna opcja. Uruchom program ponownie")
