# Grill-Me Mode

Use grill-me mode to stress-test a plan or design before implementation.

Interview the user relentlessly until you reach shared understanding. Walk through each branch of the design tree, resolving dependencies between decisions one by one. For each question, provide your recommended answer.

Ask questions with `AskUserQuestion` when it is available, batching closely related questions within the tool's limits to reduce latency. Do not answer your own questions or proceed on assumed answers unless AFK mode is also active; wait for the user's responses. If a question can be answered by exploring the codebase, explore the codebase instead.
