As of 0.4.0
# Creating A New Leader
Creating a leader has multiple steps.
- First you define the skills specific to this leader.
- Then you define which skills (specific or not) this leader has.
- Finally you create the runtime entity of the leader.

## Creating the skills

In the `assets/skill_defs.yaml` file, add any `SkillDefinition` you need.

Skill definitions are composed of multiple fields:
- An Id (defined by the `Skills` enum)
- The displayed name in game
- A name that is easier to type and always unique (friendly name)
- The description of what the skill does
- The cooldown between each use, in second
- A boolean indicating if the skill is passive (called automatically) or active (called by the player). In Shotcaller all current skills are passive (true).
- The stats conditions under which the skill will activate. This uses what is known as `Stats`, which is
a collection of values assigned to most entities (health, enemies around, mana, damage, range, etc).
- A list of required items in the inventory (item conditions) for the skill to activate.
- The effect applied to this entities' stats (doubling damage, reset cooldowns, etc)

StatConditions and effectors follow the same logic that skill definitions do:
they have an enum containing the identifiers and a collection of definitions.

## Creating a leader's skillset

A skillset is the skills a specific leader has access to.
It is created using a hashmap using the skill's key as key and the a SkillInstance as the value.
A skill instance is simply the skill's key and the current cooldown (usually 0.0).
As of 0.4.0, this is automatically done and all you need to worry about is adding your leader and all associated skills to the `assets/leader_defs.yaml` file.

## Creating a leader's entity

Leaders are regular ECS entities, with some specific components allowing for
complex behavior.
They have:
- `Point`: A position
- `Sprite`: A letter and color for drawing in the terminal
- `SpriteIndex`: A number pointing to the 2d sprite in the spritesheet used by the game. There are 10 sprites per row and 10 rows, for a total of 100 sprites.
- `Team`: Indicates which team the entity is on. Used by AI to determine which entity to attack or follow.
- `SimpleMovement`: A marker component indicating that this entity should be moved by the SimpleMovementSystem (shared by creeps and leaders).
- `Inventory`: An inventory of the items this leader has. This can easily be cloned from the default inventory (default_inventory).
- `SkillSet`: The skillset we defined earlier for this leader.
- `AiPath`: The path that this entity will follow while moving. This is used by the AI systems to move the entities around.
- `Leader`: The numeric id of the leader. This is used to select a leader with the keyboard numbers.
- `Name`: The displayed name of the leader.
- `StatSet`: The current stats of the leader. Can easily be cloned from the default statset (default_stats).
- `EffectorSet`: Holds the currently active effectors of this leader. Used to keep the `StatSet` with the correct values during gameplay.
- `FleeToBase`: Marks the health threshold at which a leader retreats back to their base.
- `IsCaught`: Tells whether or not a leader is currently unable to escape an opponent.
As of 0.4.0, this is all done in `src/systems/spawn_leader.rs`. We are not currently immediately adding the entities for leaders, so you are not required to do this at the moment. This will change once we implement a total of 10 leaders so we can have two entirely asymmetrical teams.

## Note

This will be changed, simplified and improved a lot in the future. Visit this page often to
see what changed. Have fun!
