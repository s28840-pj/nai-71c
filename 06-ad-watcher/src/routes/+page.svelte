<script lang="ts">
	import { fly, fade } from 'svelte/transition';

	import { Camera, Shield, TriangleAlert } from 'lucide-svelte';

	import AdCarousel from '$components/AdCarousel.svelte';
	import CameraFeed from '$components/CameraFeed.svelte';
	import Header from '$components/Header.svelte';
	import AttentionAlert from '$components/AttentionAlert.svelte';

	const INATTENTIVE_ALERT_THRESHOLD = 5; // seconds before showing alert

	let isMonitoring = $state(false);
	let faceDetected = $state(true);
	let isPaused = $derived(!faceDetected);
	let inattentiveSince = $state(Date.now());
	let inattentiveFor = $state(0);
	let inattentiveForSeconds = $derived(inattentiveFor / 1000);
	let totalAdsWatched = $state(0);
	let showAlert = $state(false);

	function statechanged(newState: boolean) {
		faceDetected = newState;
		if (!faceDetected) inattentiveSince = Date.now();
	}

	$effect(() => {
		if (faceDetected) return;

		const interval = setInterval(() => {
			inattentiveFor = Date.now() - inattentiveSince;
			if (inattentiveForSeconds > INATTENTIVE_ALERT_THRESHOLD) showAlert = true;
		}, 200);

		return () => clearInterval(interval);
	});
</script>

<div class="app-container scanlines">
	<Header {isMonitoring} {totalAdsWatched} />

	<main class="main-content">
		{#if !isMonitoring}
			<!-- Welcome / Start Screen -->
			<div
				class="welcome-screen"
				in:fly={{ y: 20, duration: 600, delay: 200 }}
				out:fade={{ duration: 200 }}
			>
				<div class="welcome-header">
					<div class="icon-circle">
						<Shield size={48} />
					</div>

					<h1>
						Welcome to <span class="brand-glow">AdWatch™</span>
					</h1>

					<p class="subtitle">The Future of Mandatory Advertising</p>
					<p class="quote">"Your attention is our product"</p>
				</div>

				<div class="instructions-card">
					<h2>
						<TriangleAlert size={20} class="icon-warning" />
						How It Works
					</h2>

					<ul>
						<li>
							<span class="step-number">01.</span>
							<span>We access your webcam to ensure complete attention compliance</span>
						</li>
						<li>
							<span class="step-number">02.</span>
							<span>Our AI monitors your gaze to verify you're watching the ads</span>
						</li>
						<li>
							<span class="step-number">03.</span>
							<span>Look away and the ads pause. Because you WILL watch them.</span>
						</li>
						<li>
							<span class="step-number">04.</span>
							<span>Extended inattention triggers mandatory compliance alerts</span>
						</li>
					</ul>
				</div>

				<button class="start-button" onclick={() => (isMonitoring = true)}>
					<Camera size={24} />
					Begin Mandatory Viewing
				</button>

				<p class="disclaimer">
					By clicking this button, you consent to eternal surveillance. Terms and conditions apply.
					Your attention data may be sold to the highest bidder.
				</p>
			</div>
		{:else}
			<!-- Monitoring View -->
			<div class="dashboard-grid" in:fade={{ duration: 300 }}>
				<!-- Main Content Column -->
				<div class="content-col">
					<!-- Status Banner -->
					<div
						class="status-banner"
						class:verified={faceDetected}
						in:fly={{ y: -20, duration: 300 }}
					>
						<div class="status-left">
							<div class="status-indicator">
								{#key faceDetected}
									<div class="indicator-dot" class:active={faceDetected}></div>
								{/key}
								<span class="status-text">
									{faceDetected ? 'ATTENTION VERIFIED' : 'ATTENTION REQUIRED'}
								</span>
							</div>
						</div>

						<button onclick={() => (isMonitoring = false)} class="stop-btn"> [END SESSION] </button>
					</div>

					<!-- Ad Display Area -->
					<div class="ad-zone">
						<div class="ad-header">
							<h2>Mandatory Advertisement Zone</h2>
							<span class="playback-status" class:paused={isPaused}>
								{isPaused ? '⏸ PAUSED' : '▶ PLAYING'}
							</span>
						</div>

						<AdCarousel {isPaused} adwatched={() => (totalAdsWatched += 1)} />
					</div>
				</div>

				<!-- Sidebar Column -->
				<div class="sidebar-col">
					<!-- Webcam Feed -->
					<div class="sidebar-card">
						<h3>Surveillance Feed</h3>
						<CameraFeed {statechanged} />
					</div>
				</div>
			</div>
		{/if}
	</main>

	<!-- Attention Alert Modal -->
	<AttentionAlert
		isVisible={showAlert}
		onDismiss={() => (showAlert = false)}
		inattentiveSeconds={Math.floor(inattentiveForSeconds)}
	/>
</div>

<style>
	/* --- Global Theme Variables --- */
	:global(:root) {
		--bg-background: #020617; /* Slate 950 */
		--c-primary: #ef4444; /* Red 500 */
		--c-primary-fg: #ffffff;
		--c-surveillance: #10b981; /* Emerald 500 */
		--c-warning: #f59e0b; /* Amber 500 */
		--c-muted: #64748b; /* Slate 500 */
		--c-card: #0f172a; /* Slate 900 */
		--c-border: #1e293b; /* Slate 800 */
		--c-text-main: #f8fafc; /* Slate 50 */

		--font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
	}

	/* --- Layout & Background --- */
	.app-container {
		min-height: 100vh;
		background-color: var(--bg-background);
		color: var(--c-text-main);
		position: relative;
		font-family:
			system-ui,
			-apple-system,
			sans-serif;
	}

	/* Scanline Effect */
	.scanlines::before {
		content: ' ';
		display: block;
		position: fixed;
		top: 0;
		left: 0;
		bottom: 0;
		right: 0;
		background: linear-gradient(rgba(18, 16, 16, 0) 50%, rgba(0, 0, 0, 0.25) 50%);
		background-size: 100% 4px;
		z-index: 50;
		pointer-events: none;
		opacity: 0.3;
	}

	.main-content {
		max-width: 1200px;
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	/* --- Welcome Screen --- */
	.welcome-screen {
		max-width: 42rem;
		margin: 3rem auto;
		text-align: center;
	}

	.welcome-header {
		margin-bottom: 2rem;
	}

	.icon-circle {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 6rem;
		height: 6rem;
		border-radius: 9999px;
		background-color: rgba(239, 68, 68, 0.2); /* Primary / 20 */
		margin-bottom: 1.5rem;
	}

	h1 {
		font-size: 2.5rem; /* md: 3rem */
		font-weight: 900;
		letter-spacing: -0.025em;
		margin-bottom: 1rem;
	}

	.brand-glow {
		color: var(--c-primary);
		text-shadow: 0 0 15px rgba(239, 68, 68, 0.6);
	}

	.subtitle {
		font-size: 1.25rem;
		color: var(--c-muted);
		margin-bottom: 0.5rem;
	}

	.quote {
		font-size: 0.875rem;
		color: var(--c-muted);
		font-family: var(--font-mono);
	}

	/* Instructions */
	.instructions-card {
		background-color: var(--c-card);
		border: 1px solid var(--c-border);
		border-radius: 0.5rem;
		padding: 1.5rem;
		margin-bottom: 2rem;
		text-align: left;
	}

	.instructions-card h2 {
		font-size: 1.125rem;
		font-weight: 700;
		margin-bottom: 1rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	:global(.icon-warning) {
		color: var(--c-warning);
	}

	ul {
		list-style: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		color: var(--c-muted);
	}

	li {
		display: flex;
		align-items: flex-start;
		gap: 0.75rem;
	}

	.step-number {
		color: var(--c-primary);
		font-family: var(--font-mono);
	}

	/* Start Button */
	.start-button {
		display: inline-flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 2rem;
		background-color: var(--c-primary);
		color: var(--c-primary-fg);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-radius: 0.5rem;
		font-size: 1.125rem;
		border: none;
		cursor: pointer;
		transition:
			background-color 0.2s,
			transform 0.1s;
		box-shadow: 0 0 15px rgba(239, 68, 68, 0.4);
	}

	.start-button:hover:not(:disabled) {
		opacity: 0.9;
		transform: scale(1.05);
	}

	.start-button:active:not(:disabled) {
		transform: scale(0.95);
	}

	.start-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.disclaimer {
		margin-top: 1.5rem;
		font-size: 0.75rem;
		color: var(--c-muted);
		font-family: var(--font-mono);
		line-height: 1.4;
	}

	/* --- Dashboard Grid --- */
	.dashboard-grid {
		display: grid;
		gap: 2rem;
	}

	@media (min-width: 1024px) {
		.dashboard-grid {
			grid-template-columns: 1fr 300px;
		}
	}

	.content-col,
	.sidebar-col {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	/* Status Banner */
	.status-banner {
		padding: 1rem;
		border-radius: 0.25rem;
		border: 1px solid rgba(239, 68, 68, 0.3); /* Default red border */
		background-color: rgba(239, 68, 68, 0.1);
		display: flex;
		align-items: center;
		justify-content: space-between;
		transition: all 0.3s ease;
	}

	.status-banner.verified {
		border-color: rgba(16, 185, 129, 0.3);
		background-color: rgba(16, 185, 129, 0.1);
	}

	.status-left {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.status-indicator {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.indicator-dot {
		width: 0.75rem;
		height: 0.75rem;
		border-radius: 50%;
		background-color: var(--c-primary);
		animation: blink-red 1s infinite;
	}

	.indicator-dot.active {
		background-color: var(--c-surveillance);
		animation: pulse-green 2s infinite;
	}

	.status-text {
		font-family: var(--font-mono);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		font-weight: 600;
		color: var(--c-primary);
	}

	.status-banner.verified .status-text {
		color: var(--c-surveillance);
	}

	.stop-btn {
		font-family: var(--font-mono);
		font-size: 0.75rem;
		color: var(--c-muted);
		background: none;
		border: none;
		cursor: pointer;
		text-decoration: underline;
		opacity: 0.7;
	}
	.stop-btn:hover {
		opacity: 1;
		color: var(--c-text-main);
	}

	/* Ad Zone */
	.ad-zone {
		background-color: var(--c-card);
		border: 1px solid var(--c-border);
		border-radius: 0.5rem;
		padding: 1.5rem;
	}

	.ad-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 1rem;
	}

	.ad-header h2 {
		font-size: 0.875rem;
		font-family: var(--font-mono);
		color: var(--c-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.playback-status {
		font-size: 0.75rem;
		font-family: var(--font-mono);
		color: var(--c-surveillance);
	}
	.playback-status.paused {
		color: var(--c-primary);
	}

	/* Sidebar Elements */
	.sidebar-card {
		background-color: var(--c-card);
		border: 1px solid var(--c-border);
		border-radius: 0.5rem;
		padding: 1rem;
	}

	.sidebar-card h3 {
		font-size: 0.75rem;
		font-family: var(--font-mono);
		color: var(--c-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	/* Animations */
	@keyframes blink-red {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}

	@keyframes pulse-green {
		0% {
			box-shadow: 0 0 0 0 rgba(16, 185, 129, 0.4);
		}
		70% {
			box-shadow: 0 0 0 6px rgba(16, 185, 129, 0);
		}
		100% {
			box-shadow: 0 0 0 0 rgba(16, 185, 129, 0);
		}
	}
</style>
