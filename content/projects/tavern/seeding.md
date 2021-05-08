---
title: seeding algorithms into our database
subtitle: implementing
---
While my team tackled other parts 
of Tavern, like the awesome art assets, authentication and the user interface, I was set on figuring out how we would seed algorithms.

[Alice](https://alicezhao.com) sourced a ton of algos and put them in our Notion doc prior to the hack which helped a ton. I didn't want to just hardcode them into the site, I wanted an automated and modular way that we could add algorithms as the site grew, so I first started to think about how we could represent them in a digestible way to seed our database with.

The end result is a simple addition to the build process which will iterate over a set of algorithm-specific directories in our source path to read and parse metadata, which was then inserted into our database.

An algodir contained the following files:

`metadata.json`  
This contains most of the metadata associated with a problem.
```json
{
  "id": 1,
  "name": "Two Sum",
  "description": "Write a function that takes a non-empty array of distinct integers and an integer representing a target sum. If any two numbers in the input array sum up to the target sum, the function should return them in an array, in any order. If no two numbers sum up to the target sum, the function should return an empty array.",
  "difficulty": "Easy",
  "hints": [
    "Try using two for loops to sum all possible pairs of numbers in the input array. What are the time and space implications of this approach?",
    "Realize for every number X in the input array, you are solving for Y such that X + Y = targetSum.",
    "Try storing every number in a hash table and solving for equation mentioned in hint 2 for every number and if Y is stored in the hash table. What are the time and space implications of this approach?"
  ],
  "sample": {
    "input": "[3, 5, -4, 8, 11, 1, -1, 6], 10",
    "output": "[-1, 11] // the numbers could be in reverse order"
  },
  "entrypoints": {
    "js": "twoSum($1, $2)"
  }
}
```

`testCases.json`  
This contained the test cases that we would check submitted algorithms against. Many problems don't have just one output or one input, so the schema had to be designed in a way to support multiple of both. The result is a kind of hard to decipher piece of JSON; but it worked!

```json
[
  { "input": { "0": [3, 5, -4, 8, 11, 1, -1, 6], "1": 10 }, "output": [[-1, 11], [11, -1]] },
  { "input": { "0": [4, 6], "1": 10 }, "output": [[4, 6], [6, 4]] },
  { "input": { "0": [1, 2, 3, 4, 5, 6, 7, 8, 9], "1": 17 }, "output": [[8, 9], [9, 8]] },
  { "input": { "0": [2, 7, 11, 15], "1": 9 }, "output": [[2, 7], [7, 2]] }
]
```

The last group of files in the algodir were a bunch of language-specific files in a directory called `scaffolds`. A `scaffold` is the first part of the algorithm that you see in your code editor, like the function definition and any comments, etc. We wanted to support multiple languages, so it was important that the scaffold system supported that. Scaffolds were mapped to a language by their file extension.

`scaffold.js`  
```js
function twoSum(array, target) {

}
```

An added benefit of this whole system was that our algorithms were now backed by VCS. We had designed a system where users could submit their own algorithms as pull requests to our open source repository which would be implemented as soon as they were merged as the seeding is all a part of the build process. 