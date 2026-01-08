<script lang="ts">
	import { Eye, Radio } from 'lucide-svelte';

	interface Props {
		isMonitoring: boolean;
		totalAdsWatched: number;
	}

	let { isMonitoring, totalAdsWatched }: Props = $props();
</script>

<header class="header">
	<div class="container">
		<div class="header-content">
			<!-- Logo Section -->
			<div class="logo-group">
				<div class="icon-wrapper" class:monitoring={isMonitoring}>
					<Eye size={24} />
				</div>
				<div class="title-wrapper">
					<h1>
						AdWatch<span class="tm">™</span>
					</h1>
					<p class="subtitle">ATTENTION ENFORCEMENT SYSTEM</p>
				</div>
			</div>

			<!-- Status Indicators -->
			<div class="status-group">
				<!-- Live Indicator -->
				<div class="live-indicator">
					<div class="dot" class:monitoring={isMonitoring}></div>
					<span class="status-text">
						{isMonitoring ? 'LIVE' : 'OFFLINE'}
					</span>
				</div>

				<!-- Stats (Hidden on mobile via CSS) -->
				<div class="stats-display">
					<div class="stat-item">
						ADS CONSUMED: <span class="highlight">{totalAdsWatched}</span>
					</div>
					<div class="broadcast-badge">
						<Radio size={12} />
						<span>BROADCASTING</span>
					</div>
				</div>
			</div>
		</div>
	</div>
</header>

<style>
	/* --- Variables (matching the Index component theme) --- */
	header {
		--c-primary: #ef4444; /* Red */
		--c-muted: #64748b; /* Slate 500 */
		--c-muted-bg: #1e293b; /* Slate 800 */
		--c-card-bg: rgba(15, 23, 42, 0.85); /* Slate 900 with opacity */
		--c-border: #1e293b;
		--c-text: #f8fafc;
		--c-surveillance: #10b981; /* Green */

		--font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
	}

	/* --- Layout --- */
	.header {
		position: sticky;
		top: 0;
		z-index: 40;
		width: 100%;
		border-bottom: 1px solid var(--c-border);
		background-color: var(--c-card-bg);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		color: var(--c-text);
	}

	.container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 1rem;
	}

	.header-content {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	/* --- Logo Section --- */
	.logo-group {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.icon-wrapper {
		padding: 0.5rem;
		border-radius: 0.375rem;
		background-color: var(--c-muted-bg);
		color: var(--c-muted);
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.3s ease;
	}

	/* Monitoring State for Icon */
	.icon-wrapper.monitoring {
		background-color: rgba(239, 68, 68, 0.2); /* Primary / 20 */
		color: var(--c-primary);
		animation: heartbeat 1s infinite;
	}

	.title-wrapper h1 {
		font-size: 1.25rem;
		font-weight: 900;
		letter-spacing: -0.025em;
		margin: 0;
		line-height: 1.2;
	}

	.tm {
		color: var(--c-primary);
	}

	.subtitle {
		font-size: 0.75rem;
		color: var(--c-muted);
		font-family: var(--font-mono);
		margin: 0;
	}

	/* --- Status Section --- */
	.status-group {
		display: flex;
		align-items: center;
		gap: 1.5rem;
	}

	/* Live Indicator */
	.live-indicator {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.dot {
		width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
		background-color: var(--c-muted);
		opacity: 0.3;
		transition: background-color 0.3s ease;
	}

	.dot.monitoring {
		background-color: var(--c-primary);
		opacity: 1;
		animation: blink 1s infinite;
	}

	.status-text {
		font-size: 0.75rem;
		font-family: var(--font-mono);
		color: var(--c-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Stats Display (Desktop) */
	.stats-display {
		display: none; /* Hidden on mobile */
		align-items: center;
		gap: 1rem;
		font-size: 0.75rem;
		font-family: var(--font-mono);
	}

	@media (min-width: 640px) {
		.stats-display {
			display: flex;
		}
	}

	.stat-item {
		color: var(--c-muted);
	}

	.highlight {
		color: var(--c-text);
		font-weight: 700;
	}

	.broadcast-badge {
		display: flex;
		align-items: center;
		gap: 0.25rem;
		color: var(--c-surveillance);
	}

	/* --- Animations --- */
	@keyframes heartbeat {
		0%,
		100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.1);
		}
	}

	@keyframes blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}
</style>
