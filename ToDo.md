# WIP Version

# General
## ToDo
- Refactor combat system
    - ranged en melee proper uit elkaar halen (aparte systemen?)
    - if isplayer werkt enkel als enkel de player aangevallen wordt!!!
## Done
- git version control

## Features
### ToDo
1. stacked (amount) projectiles
    - (X) base
    - (X) print stack in hud
    - () add to stack when equiped
2. targeting (check rustyrogueliketutorial)
    - (X) targetable component for creatures
    - (X) targetting component for player
    - () playerinput 'tab' sends CycleTargetting messages (van Max   tot len)
    - targettingsystem
        - (X) checks CycleTargetting message
        - (X) querys tragetables in playerfov
        - (X) targets next index
3. make entity theme for tileset (similar to maptheme)
4. animate projectiles
### Done
- Kenney Tiles test?


## Balance
### ToDo

### Done


## Bugs
### ToDo
### Done
- equiped items don't stay on levelchange


## Stretch Goals

1. turn_initiative (check rustyrogueliketutorial)
2. ai chaning (check rustyrogueliketutorial)
3. Feromone attraction ai
4. equipment system with slots (check rustyrogueliketutorial)

### Bag
- coating of fruit on weapons
- worms multiply / spawn in group
- more complex behaviour of creatures
- more (and more interesting) maps
- list of seen items/plants/creatures
- observation system
    - interaction/observation screen (shows a graphic, describes creature/plant/environment/behaviour)
        - with ascii art
- visually more varied and appealing environments
    - pools
- alert alignment object (creatures turn on others, alignmentchange)

- implementeer eerst in het bestaande systeem een aantal elementen van de sfeer
- chase ai rattten, randomai worms, amiguai voor mieren
- kijk eens naar interessante maplayouts
- herringbone wang tiles
- gebruik dit voor het plaatsen van minizones
- Je kan ook normaalverdeling gebruiken
- evolueer het ecosysteem van het level over een bepaalde tijd
- variabel initiatief systeem voor acties
- central mechanic?
- Wat voor soort gameplay zou je hebben in zo'n ecosysteem-dungeon?
- . , ; als stadia van grassige begroeiing => eten voor creatures
- evolutionair algorithme
- geen xp (ook niet voor discovery)
- states als componenten(met een severity)
- groupscomponenten die overwriten