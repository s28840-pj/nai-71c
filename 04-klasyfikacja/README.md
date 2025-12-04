# Klasyfikacja

# Dokładność modeli
## Zestaw Wine Quality
```
Drzewo decyzyjne:

classes    | niedobre   | dobre
niedobre   | 1345       | 37
dobre      | 38         | 179

dokładność 0.9530957, MCC 0.7996681

SVM (Kernel gaussowski):

classes    | niedobre   | dobre
niedobre   | 1290       | 92
dobre      | 3          | 214

dokładność 0.9405879, MCC 0.8006177

SVM (Kernel wielomianowy):

classes    | niedobre   | dobre
niedobre   | 1249       | 133
dobre      | 139        | 78

dokładność 0.8298937, MCC 0.26634818
```

## Zestaw Heart Disease
```
Drzewo decyzyjne:

classes    | zdrowy     | chory
zdrowy     | 131        | 7
chory      | 13         | 152

dokładność 0.9339934, MCC 0.8680889

SVM (Kernel gaussowski):

classes    | zdrowy     | chory
zdrowy     | 138        | 0
chory      | 0          | 165

dokładność 1, MCC 1

SVM (Kernel wielomianowy):

classes    | zdrowy     | chory
zdrowy     | 106        | 32
chory      | 30         | 135

dokładność 0.7953795, MCC 0.5870459
```
