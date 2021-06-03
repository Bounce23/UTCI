# Heat Stress
This is a pure rust implementation of a new algorithm,
to calculate the amount of experienced Heat Stress within
the urban area of Groningen, the Netherlands.
This work is a component to a citizen science campaign,
which intends to empower citizens of Groningen with the 
tools and understanding to identify, prevent and treat
the effects of heat stress.

This library utilises an optimised version 
of the UTCI with calculation of the radiant 
heat sources from differing urban environments.
The Town Energy Balance Model is used to understand
the effects of radiation from various emissivities 
of differing materials. These are combined, along 
with the biological traits of vegetation to output
a one-dimensional result to the amount of Heat Stress
felt. 

# Inputs 
The parameters for the modelling are:
- Air Temperature
- Wind Speed
- Relative Humidity
- Are you situated in the city centre(dutch: Binnenstad)?
- Are you under the shade of a tree? 
- Are you situated in a park?

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
to the below scale, which allows for quantification 
of Experienced Heat Stress.
This Scale ranges from -50[c] (Extreme Cold
Stress), to +50[c] (Extreme Heat Stress).

