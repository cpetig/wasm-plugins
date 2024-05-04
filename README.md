# Plugin experiments

The subdirectories contain:

- app: Application, composes with plugin1, uses plugin world
- app2: Application, composes with plugin1 and plugin3, uses plugin and plugin2 world
- plugin1: example implementation of plugin world
- plugin2: another implementation of plugin world
- plugin3: implements plugin2 world
- adapter: imports plugin world, exports plugin2 world

App2 will compose with two adapters and print their names.

If you compose adapter with plugin2 it will make plugin2 compatible with app2, so that app2 can link to plugin1 and plugin2. [Thanks to Bailey Hayes for the inspiration!]

There is also a "broken" branch which tries to use a common interface between the plugins and breaks.