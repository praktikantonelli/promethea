# Domain

Important: According to Eric Evans' "Domain-Driven Design", it is vital that the design of the domain model and the development is not decoupled. In other words, while it makes sense to come up with a basic domain model to start with, the model can and should evolve over time. 

## Event Storming
Event storming is a process that can be used to try and create a domain model. 

## Domain Experts
One challenge of domain-driven design will be the lack of "domain experts". In an example, Evans is the software engineer and he works with a group of electrical engineers that design PCBs. He states how important it is to have this exchange between programmers and domain experts, for example to establish a common language and a model that works for both sides of the problem.

Given that I'm doing this alone, for myself, I will have to take on the role of both engineer and domain expert. 

*If sophisticated domain experts don't understand the model, there is something wrong with the model.* - Eric Evans

## Model Refinement
In order to improve on a domain model's refinement, one can try and talk out loud about the various constructs because when speaking about entities and their relationships, we realize which parts of the relationships need to be improved on. Rough models will include more vague terms and names that may need to be made more concise with specific terms. Generally experimenting with language can help here. 

## Ubiquitous Language
The book mentions the concept of ubiquitous language as meaning that the model should contain terms and language such that a domain expert can use the words used in the model to specifically talk about concise use cases of the application. It should be easy to speak about the domain using those terms, without it being awkward to use. 

If a part of the model seems awkward to use, it may be a sign that the nomenclature could be improved upon.

## Modules
Instead of splitting the code into modules grouped by level (e.g., entities, services, ...), the code should be split by cohesive concepts, such as library, reading tracking, metadata etc.
