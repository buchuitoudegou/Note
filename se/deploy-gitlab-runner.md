# How to deploy a gitlab-runner
## procedure
+ sudo gitlab-runner register
+ input the `URL` and `Token` of the project. (in Project->settings->CI/CD)
+ input tags (or not)
+ input descriptions (the name of your runner)
+ select a executor (e.g. shell, docker, ssh+docker, etc)
+ sudo gitlab-runner start
## common mistakes
+ `New Runner has not been connected yet.` remove your runner in the webpage and create a new runner using other executor such as `shell`
+ `Job is stuck. Check your runner.` This means the runner doesn't pick your project. There is probably no tags in your `.gitlab-ci.yml` while your runner needs some tags. Remove the runner and create a new runner without tags (press `enter` to skip that step) or tag your job in `.gitlab-ci.yml`.