# Design

## Goals

The goal of the [jupyterlab/rtc](https://github.com/jupyterlab/rtc) repository is to prototype a data model to support simultaneous distributed
editing in Jupyter. Fundamentally, it's about taking the **existing** concepts [^f1] provided by Jupyter Server and creating a `near` real time data model on top of them.

This would help provide a Google Docs like editing experience in Jupyter editors by allowing multiple simultaneous users to edit a document at once.

It would do this by moving much of the state to be shared on the server as well, which would also help address some other long-running issues with Jupyter, around preserving outputs even after you close your browser and dealing with race conditions around kernel management. This will happen in a number of different layers, added to the [jupyterlab/rtc](https://github.com/jupyterlab/rtc) monorepo:

1. Top-level layer developed in the `Applications`: [JupyterLab](https://github.com/jupyterlab/jupyterlab), [Spyder](https://www.spyder-ide.org/), [Nteract](https://github.com/nteract/nteract)...
1. High-level layer, a.k.a `Jupyter` components that support editing all data in Jupyter server.
1. Middle-level layer that expose friendly Real Time data access using `React.js` integration.
1. Base-level layer that support CRDT implementations ([Lumino](https://github.com/jupyterlab/lumino) and/or [other CRDT libraries](/about-rtc/algorithms.html#crdt)).

The definition of those layers is still in progress. You can discuss this on [jupyterlab/rtc#61](https://github.com/jupyterlab/rtc/issues/61).

## Non Goals

There are also a number of other ways to improve collaboration in Jupyter which the [jupyterlab/rtc](https://github.com/jupyterlab/rtc) repository is not meant to address, at least initially.

- It is not meant to be able to synchronize notebooks or work between Jupyter servers running on multiple machines.
- It doesn't overlap with JupyterHub in that JupyterHub is about spawning multiple instances and this is about providing one instance for collaborative purposes.
- It also isn't about providing concepts of Teams, Projects, Groups, etc. These are definitely useful to have in a collaborative environment but for the initial work here we are assuming the same permission-less structure of a regular Jupyter server.
- It's also often useful to share what environment you are in with your notebook, to make it more reproducible. This isn't about that kind of collaboration either.

[^f1]: Jupyter Server specifications.

    - [Kernel Messaging](https://jupyter-client.readthedocs.io/en/stable/messaging.html)
    - [Notebook Server Swagger API](http://petstore.swagger.io/?url=https://raw.githubusercontent.com/jupyter/notebook/master/notebook/services/api/api.yaml)
    - [Notebook Server Wiki](https://github.com/jupyter/jupyter/wiki/Jupyter-Notebook-Server-API)
    - [Next Jupyter Server](https://jupyter-server.readthedocs.io)
