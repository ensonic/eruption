id = '5dc65fa6-e965-45cd-a0da-e87d29713123'
name = 'Color Swirls (Simplex): Rainbow'
description = 'Color Swirl effect'
active_scripts = [
    'swirl-simplex.lua',
    'halo.lua',
    'shockwave.lua',
#   'impact.lua',
#   'water.lua',
#   'raindrops.lua',
#   'sysmon.lua',
    'batique.lua',
#   'dim-zone.lua',
    'macros.lua',
#   'stats.lua',
]

[[config."Simplex Swirl"]]
type = 'float'
name = 'color_divisor'
value = 1.0
default = 1.0

[[config."Simplex Swirl"]]
type = 'float'
name = 'color_offset'
value = 0.0
default = 0.0

[[config."Simplex Swirl"]]
type = 'float'
name = 'time_scale'
value = 200.0
default = 200.0

[[config."Simplex Swirl"]]
type = 'float'
name = 'coord_scale'
value = 18.0
default = 18.0

[[config.Shockwave]]
type = 'color'
name = 'color_step_shockwave'
value = 0x05010000
default = 0x05010000

[[config.Shockwave]]
type = 'bool'
name = 'mouse_events'
value = true
default = true

[[config.Shockwave]]
type = 'color'
name = 'color_mouse_click_flash'
value = 0xa0ff0000
default = 0xa0ff0000

[[config.Shockwave]]
type = 'color'
name = 'color_mouse_wheel_flash'
value = 0xd0ff0000
default = 0xd0ff0000

[[config.Raindrops]]
type = 'float'
name = 'opacity'
value = 0.75
default = 0.75

[[config."System Monitor"]]
type = 'color'
name = 'color_cold'
value = 0x0000ff00
default = 0x0000ff00

[[config."System Monitor"]]
type = 'color'
name = 'color_hot'
value = 0xffff0000
default = 0xffff0000

# mouse support
[[config.Batique]]
type = 'int'
name = 'zone_start'
value = 144
default = 144

[[config.Batique]]
type = 'int'
name = 'zone_end'
value = 180
default = 180

[[config.Batique]]
type = 'float'
name = 'coord_scale'
value = 0.5
default = 0.5

# dim a specific zone, e.g. if the mouse LEDs are too bright
[[config."Dim Zone"]]
type = 'int'
name = 'zone_start'
value = 144
default = 144

[[config."Dim Zone"]]
type = 'int'
name = 'zone_end'
value = 180
default = 180

[[config."Dim Zone"]]
type = 'float'
name = 'opacity'
value = 0.95
default = 0.95