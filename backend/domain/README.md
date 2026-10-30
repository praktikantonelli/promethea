# Domain Model
This library contains the domain model of the project. Its purpose is to define business logic without any dependencies to anything else, apart from very few low-level utility libraries (e.g., error handling, date and time etc.).

The goal is that the domain model forms the core of the application. Inbound and outbound requests can and should depend on it, but not vice versa!
