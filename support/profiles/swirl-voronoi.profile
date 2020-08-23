id = '5dc62fa6-e965-45cb-a0da-e87d29713115'
name = 'Color Swirls (Voronoi)'
description = 'Color Swirl effect'
active_scripts = [
    'swirl-voronoi.lua',
    'shockwave.lua',
#   'impact.lua',
#   'water.lua',
#   'raindrops.lua',
#   'sysmon.lua',
    'macros.lua',
#   'stats.lua',
#   'profiles.lua',
]

[[config."Voronoi Swirl"]]
type = 'float'
name = 'time_scale'
value = 50.0

[[config."Voronoi Swirl"]]
type = 'float'
name = 'coord_scale'
value = 5.0

[[config.Shockwave]]
type = 'color'
name = 'color_step_shockwave'
value = 0x05010000

[[config.Shockwave]]
type = 'bool'
name = 'mouse_events'
value = true

[[config.Shockwave]]
type = 'color'
name = 'color_mouse_click_flash'
value = 0xa0ff0000

[[config.Shockwave]]
type = 'color'
name = 'color_mouse_wheel_flash'
value = 0xd0ff0000

[[config.Raindrops]]
type = 'float'
name = 'opacity'
value = 0.75

[[config."System Monitor"]]
type = 'color'
name = 'color_cold'
value = 0x0000ff00

[[config."System Monitor"]]
type = 'color'
name = 'color_hot'
value = 0xffff0000
