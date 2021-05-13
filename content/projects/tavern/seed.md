---
title: 🌞🚰🌱
subtitle:
---
we did a lot of planning leading up to this project. and when i say *we* i mainly mean everybody else, with a special shout out to [alice](https://alicezhao.com), who somehow managed to do a shit ton of planning, market research, and source all our algos (god i wish you could've see the notion doc 🤤🤤) and then code like the entire fucking authentication system (all while being very new to web development). absurd. 

anyways lmao, while [anthony](https://anthonymorris.dev) was shitting out the sexiest freestyled frontend code ive ever seen at an insane speed, i had to figure out how tf we were gon get these algos on the site.

we coulda hardcoded them into the site. honestly, in retrospect, not even a bad option. we had 24 hours to make some shit popping so every minute saved woulda been good. but we wanted to make something the same way we'd make it if we were actually trying to bring the product to market. this meant the shit **had** to be extensible, it **had** to be easy to add algos and it **had** to be easy to change them. so we decided it'd make more sense to shove the algos into the db. 

i had to take all the shit attached to an algo, like the name, test cases, examples, hints, etc, and get it into a format that would make it not only easily digestable by a database, but also by humans. it needed to be easy and straight forward to add algos.

i went through a couple diff ideas on how to impl this, like just having a bunch of api routes and hitting them manually to seed db, etc. but then we'd have to worry ab authenticating those routes, which means adding more and more layers of security to ensure user has correct permissions to create stuff, etc. 

i opted instead for a **fs-backed templating system** for seeding algorithms. 

as a part of the build process of the backend component of tavern, a build script would iterate over subdirectories within an `algorithms` directory and read the contents of specific files to build an internal representation of that algorithm to then insert into the database. each algodir (algorithm directory) contains a `metadata.json` file. this file has all the static information pertaining to an algo problem, like its name, description, difficulty, etc. it also contains the 'entrypoint' for a specific language's execution of the problem. some languages have different syntax for the invocation of a function, so we had to design a system that would support that.    

example `metadata.json` for the `twoSum` problem:
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
    "js": "twoSum($1, $2)",
    "py": "two_sum($1, $2)"
  }
}
```

each algodir also contains a `testCases.json` file, which if u can guess, houses all the testcases that'll we check a users algo against to determine whether it is successful or not. different algos have diff outputs, they don't all return just a single number. the schema for this file had to be designed to support multiple inputs and multiple outputs, the result is a very weird looking piece of json and the least human-digestable file in the codebase:

```json
[
  { "input": { "0": [3, 5, -4, 8, 11, 1, -1, 6], "1": 10 }, "output": [[-1, 11], [11, -1]] },
  { "input": { "0": [4, 6], "1": 10 }, "output": [[4, 6], [6, 4]] },
  { "input": { "0": [1, 2, 3, 4, 5, 6, 7, 8, 9], "1": 17 }, "output": [[8, 9], [9, 8]] },
  { "input": { "0": [2, 7, 11, 15], "1": 9 }, "output": [[2, 7], [7, 2]] }
]
```

finally, each algodir also contains a subdirectory called `scaffolds`. a *scaffold* is essentially the boilerplate function definition + any comments, etc, for a given algo. because we wanted to support multiple programming languages, we needed a system that would support multiple scaffolds. 

scaffold for javascript two sum (`scaffold.js`):
```js
function twoSum(array, target) {

}
```

scaffold for python two sum (`scaffold.py`):
```python
def two_sum(array, target):
  pass
```

a scaffold is what a user first sees in tavern's code editor when they load up an algo and select a language.

the *true* benefit to all of this, is that we had created a system where our algos were backed by version control. ppl could clone our repo, create their own algos by extending the simple templating system, and then open up a pull request which upon approval, CI could run our build script and seed the algo straight into our prod db. 