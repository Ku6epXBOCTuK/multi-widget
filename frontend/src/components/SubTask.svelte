<script lang="ts">
	import type { SubTask, TaskStatus } from "$lib/generated-bindings";

	interface Props {
		subtask: SubTask;
	}

	let { subtask }: Props = $props();

	interface IStatusConfig {
		icon: string;
	}

	const statusConfig: Record<TaskStatus, IStatusConfig> = {
		pending: {
			icon: "[ ]",
		},
		"in-progress": {
			icon: "[/]",
		},
		done: {
			icon: "[✓]",
		},
	} as const;

	let { icon } = $derived(statusConfig[subtask.status]);
</script>

<div class="subtask">
	<span class="icon {subtask.status} mono-font">{icon}</span>
	<span class="title {subtask.status}">
		${subtask.title} #{subtask.id}
	</span>
</div>

<style>
	.subtask {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.icon {
		font-size: 0.75rem;
		margin-top: 0.125rem;

		&.pending {
			color: var(--text-dim);
		}

		&.done {
			color: var(--accent-green);
		}

		&.in-progress {
			background: linear-gradient(
				90deg,
				#d97706 0%,
				#fcd34d 40%,
				#ffffff 50%,
				#fcd34d 60%,
				#d97706 100%
			);
			background-size: 200% auto;
			background-clip: text;
			-webkit-background-clip: text;
			-webkit-text-fill-color: transparent;
			animation: shimmer 2.5s linear infinite;
		}
	}

	.title {
		font-size: 0.75rem;
		color: var(--text-muted);
		transition: color 0.3s;
		&.done {
			color: #6b7280;
			text-decoration: line-through;
		}
	}

	@keyframes shimmer {
		0% {
			background-position: 200% center;
		}
		100% {
			background-position: -200% center;
		}
	}
</style>
