select: `{gender, select,
	male {{animalName} the {animalType} loves his friends}
	female {{animalName} the {animalType} loves her friends}
	other {{animalName} the {animalType} loves their friends}
}`

plural: `{legCount, plural,
	=0 {{animalName} the {animalType} has no legs}
	one {{animalName} the {animalType} has # leg}
	other {{animalName} the {animalType} has # legs}
}`

offset: `{octopusCount, plural, offset:2
	=0 {All octopuses are accounted for}
	=1 {{octopusName} has escaped through the drain!}
	=2 {{octopusName} and {octopus2Name} have escaped through the drain!}
	one {{octopusName}, {octopus2Name}, and # other octopus have escaped through the drain!}
	other {{octopusName}, {octopus2Name}, and # other octopuses have escaped through the drain!}
}`

selectordinal: `{rank, selectordinal,
	=1 {The {animalType} is the largest animal in the ocean}
	one {The {animalType} is the #st largest animal in the ocean}
	two {The {animalType} is the #nd largest animal in the ocean}
	few {The {animalType} is the #rd largest animal in the ocean}
	other {The {animalType} is the #th largest animal in the ocean}
}`

nested: `{animalHabitat, select,
	ocean {{rank, selectordinal,
		=1 {The {animalType} is the largest animal in the ocean}
		one {The {animalType} is the #st largest animal in the ocean}
		two {The {animalType} is the #nd largest animal in the ocean}
		few {The {animalType} is the #rd largest animal in the ocean}
		other {The {animalType} is the #th largest animal in the ocean}
	}}
	land {{rank, selectordinal,
		=1 {The {animalType} is the largest animal on land}
		one {The {animalType} is the #st largest animal on land}
		two {The {animalType} is the #nd largest animal on land}
		few {The {animalType} is the #rd largest animal on land}
		other {The {animalType} is the #th largest animal on land}
	}}
	other {{rank, selectordinal,
		=1 {The {animalType} is the largest flying animal}
		one {The {animalType} is the #st largest flying animal}
		two {The {animalType} is the #nd largest flying animal}
		few {The {animalType} is the #rd largest flying animal}
		other {The {animalType} is the #th largest flying animal}
	}}
}`

split: `There {thingCount, plural, =0 {are no} other {are #} one {is #}} {type, select, other {"uncool"} neutral {"mid"} good {"cool"}} things at {name}'s house.`

dateNumber: `{animalName} the octopus’s birthday is on {birthday, date}. He invited {inviteCount, number} friends to his party, but only {acceptedShare, number, ::percent} can make it.`

skeleton: `{animalName} the octopus’s birthday is on {birthday, date, ::cccccMMMMd}. He invited {inviteCount, number, ::K} friends to his party, but only {acceptedShare, number, ::percent .0} can make it.`

escape: `An {argument}'s '{escaped}' with #'' '#'; '<b>'{notEscaped} #'</b>' '<' '}' '' <notEscaped> > '{}' '<>'`
