import type { Stage } from "./generated-bindings";

export const stages: Stage[] = [
	{
		id: 1,
		title: "Stage 1",
		status: "done",
		tasks: [
			{
				id: 11,
				parent: 1,
				title: "Task 1",
				description: "This is task 1",
				status: "done",
				subtasks: [
					{
						id: 111,
						parent: 11,
						title: "Subtask 1",
						status: "done",
						description: "This is subtask 1",
					},
					{
						id: 112,
						parent: 11,
						title: "Subtask 2",
						status: "in-progress",
						description: "This is subtask 2",
					},
					{
						id: 113,
						parent: 11,
						title: "Subtask 3",
						status: "pending",
						description: "This is subtask 3",
					},
				],
			},
			{
				id: 12,
				parent: 1,
				title: "Task 2",
				description: "This is task 2",
				status: "in-progress",
				subtasks: [],
			},
			{
				id: 13,
				parent: 1,
				title: "Task 3",
				description: "This is task 3",
				status: "pending",
				subtasks: [],
			},
		],
	},
] as const;
