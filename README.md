## About

Carbon is a CLI tool that enables you to track the time you spend on different projects efficiently. It allows you to start, pause, resume, and stop time tracking on individual projects and provides you with summaries of the time spent on each project.

## Usage

To interact with the Carbon CLI, you will be using the following commands:

### Start

To start tracking time on a new project, use the `start` command followed by the title of your project. 

`carbon start --title "Your Project Title"`

### Pause

To pause tracking time on an existing project, use the `pause` command followed by the ID of your project.

`carbon pause YOUR_PROJECT_ID` 

### Resume

To resume tracking time on a paused project, use the `resume` command followed by the ID of your project.

`carbon resume YOUR_PROJECT_ID` 

### Stop

To stop tracking time on an existing project, use the `stop` command followed by the ID of your project.

`carbon stop YOUR_PROJECT_ID` 

### Status

To view the status of the currently running projects, use the `status` command.

`carbon status` 

### List

To list all, today's or yesterday's completed projects, use the `list` command followed by a subcommand (`all`, `today`, or `yesterday`).

`carbon list all`
`carbon list today`
`carbon list yesterday` 
