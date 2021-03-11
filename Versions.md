# 210310
## Cleanup & Balancing
- remove println's
- reset hp to 10
- balancing + poison dart
- tooltip layer shows target
## General
- end_input_system implemented
- inputsystem refactor, no delta's, fn's
- git version control
## Features
-stacked (amount) projectiles
    - (X) base
    - (X) print stack in hud
-targeting (check rustyrogueliketutorial)
    - (X) targetable component for creatures
    - (X) targetting component for player
    - (X) playerinput 'tab' sends CycleTargetting messages (van Max   tot len)
    - targettingsystem
        - (X) checks CycleTargetting message
        - (X) querys tragetables in playerfov
        - (X) targets next index
- Kenney Tiles test?


# 210308 MVP!
- basic content 
    - (X) 3 creatures 
    - (X) 3 kinds of ai
    - (X) 3 plants
        - (X)giving the fruit itmes
    - (X) 3 use_items
        - (X) healing
        - (X) mapping
        - (X) combo
    - (X) 3 equipment_items
        - (X) melee weapon
        - (X) ranged weapon
        - (X) armour
    - (X) basic game balancing (1/3 win)
- basic equipment system
    - use to equip
    - overwrite and remove same kind
    - equipment affects combat
- implement basic theme (16bit color pallet)
- ascii 'pseudo-graphics'
- hands on rust finished
## bugfixes
- equiped items don't stay on levelchange