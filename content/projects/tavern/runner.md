---
title: code runner
subtitle: implementing
---
We needed a way to execute user submitted code against a subset of testcases so we could verify whether a solution was valid or not. There are a ton of different ways this can be accomplished. I think if we had more time the ideal solution would have been to have a language-specific Docker image that we could execute on a given submition. This would ensure that a user's code could not poison the site environment as it'd be completely isolated within the container. Seeing as the hackathon was only 24 hours long, we opted, instead, to find a different solution that we could implement very quickly. We ended up using the [vm2](https://github.com/patriksimek/vm2) package, which is essentially a sandbox or a virtual machine. We'd run the user's code inside of the virtual environment, after we overwrote some of the functions internally (like `console.log`) so that we could capture the output and show it in our frontend. A caveat of this approach, unfortunately, was because the program ran in a pseudoenv, the stacktraces contained references to the calling functions and attempts to sanitize it were mostly futile. It was enough for users to debug their code though, so we moved on.

We had successfully implemented a code runner that would:
1. Process and execute user submitted code
2. Plug in test case variables into the function parameters from the user submmit code & scaffold
3. Capture all prints to display to the user who submitted
4. Capture and display any and all stacktraces so that users could debug their code
5. Determine whether a given solution passed every testcase and confirm whether a solution was correct
6. Record & display the execution time of a given solution so that users could optimize their code