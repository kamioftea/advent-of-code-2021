initSidebarItems({"fn":[["initialisation_limit","The initialisation phase (part_one) is limited to a cube 50 units from the origin on all axes."],["limit_instructions","Filter the list of instructions to just the region that intersects the limit [`Cuboid`]. If an instruction’s cuboid is partially in the area, instead include a modified instruction that just contains the intersection with the limit."],["merge_instruction","Merge an instruction into the current list of cuboids. Use [`Cuboid::diff_and_split`] to remove the instruction’s cuboid from other cuboids it overlaps. Then if it is itself on, add the new cuboid to the list to mark that its entire region is now active."],["parse_input","Parse the puzzle input as a list of instructions"],["run","The entry point for running the solutions with the ‘real’ puzzle input."],["volume_active","Fold the list of instructions into a list of cuboids that describe the entire active area, then sum the volumes of those cuboids to get the total active volume."]],"struct":[["Cuboid","Represents a cuboid as its range of co-ordinates on each axis. Both values are inclusive."],["Instruction","Represents a line of input as the [`Cuboid`] region it intersects, and whether it toggles its contents on or off."]]});