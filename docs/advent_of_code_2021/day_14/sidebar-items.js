initSidebarItems({"fn":[["intersperse","The name is a legacy from the naive solution where this was mapping each pair to the new pairs and building the full polymer in order which failed as the final polymer had ~21 trillion characters. This takes all the existing pairs and adds their counts to the pair(s) they map to when the mapped character is inserted."],["into_count_by","Reduce the pair mapping into a count of characters. This needs to be called twice once for each element in the pair, to account for the first and last character that are each only in one pair. The mapping parameter is to capture this difference, and maps a pair count entry from the Polymer into the character this invocation cares about"],["into_pair_counts","Split a list of characters into the counts of all the consecutive pairs that exist. The hard work is delegated to library functions [`slice::windows`] to give an iterator of the pairs and [`Itertools::counts`] to reduce that to the required map."],["iterate","Recursively apply [`intersperse`] the required number of times"],["parse_input","The types required to make today’s solution work are pretty complex, so there is quite a lot of work here to take a relatively simple input format into the complex format that makes the logic efficient. A bunch of the tests need to convert intermediate polymer string representations into the map of pair counts used internally, so this is delegated to [`into_pair_counts`]."],["polymer_length",""],["run","The entry point for running the solutions with the ‘real’ puzzle input."],["summarise","This is responsible for converting the internal representation of a polymer into the data needed to provide the puzzle solution. It also returns the intermediary hashmap so that this can be verified in tests against the example provided in the specification."]],"type":[["PairMap","The internal representation of the insertion map, that returns the two new pairs generated by inserting the specified character."],["Polymer","The internal representation of polymer as the counts of the distinct consecutive pairs."]]});