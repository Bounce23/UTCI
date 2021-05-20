# UTCI
This is a pure rust implementation of the 
Universal Temperature Climate Index calculator, 
which provides a quantification and result to 
an amount of Heat Stress. 

This library utilises an optimised version 
of the UTCI with Fiala model, to provide 
a mathematical one-dimensional result to 
the amount of Heat Stress Felt. 

# Inputs 
The parameters for the modelling are:
- Air Temperature
- Radiant Temperature
- Wind Speed
- Relative Humidity

# Quantification 
The mathematical formulae for calculating 
Heat Stress, uses a polynomial regresssion 
model based on the nature of the inputs and 
their relationship to one another. This allows 
for the resulting single dimension outputs, 
such that we can scale and compare experienced
heat stress. 

# Output
The output is a decimal number, which correlates 
to a scale for UTCI equivalent temperature. 
This Scale ranges from -50[c] (Extreme Cold
Stress), to +50[c] (Extreme Heat Stress).

