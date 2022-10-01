# Osmosis Dynamic Incentives primitive

HackWasm Medellin 2022 project -  
Osmosis Challenge -  
DeFi Primitive -  
  
## Abstract

Osmosis standard Incentives module provides a general method to give rewards to stakers. 

The yield to be given to stakers are stored in a gauge and it is distributed on epoch basis to the stakers who meet specific conditions.

There are two kinds of gauges:

* Non perpetual gauge get removed from active queue after the the distribution period finish 
* Perpetual gauge  distribute all the tokens at a single time and somewhere else put the tokens regularly to distribute the tokens

We want to create a CosmWasm Smart contract that replenish the (Perpetual ?) Gauge on a daily base according to several dynamic market conditions.

This module objective is to foster price and liquidity size over time with dynamic incentives.

On our fist version we are allocating 2 kind of incentives:

* LP Base incentive 
* LP Size Bonus incentive
* Custom made incentive (Later versions)


## Base incentive

This is the basic yield and does not dynamically change . It provides the core APY on a daily base 


##  LP Size Bonus incentive

This yield dynamically changes according to the LP Size. Extra Bonus Incentives are automatically added if LP size decreases. This bonus should  motivate people to buy tokens and add a LP

