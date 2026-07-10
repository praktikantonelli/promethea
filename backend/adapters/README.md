# Adapters

There should generally be one adapter per port defined in [`shared/src/ports`](../shared/src/ports). Each adapter is a concrete implementation of a generic port, so any 3rd-party dependency should ideally go in here (apart from some specific exceptions). 


