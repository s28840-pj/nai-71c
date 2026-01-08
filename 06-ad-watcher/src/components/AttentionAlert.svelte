<script lang="ts">
	import { fade, scale } from 'svelte/transition';
	import { cubicOut } from 'svelte/easing';
	import { TriangleAlert, Volume2 } from 'lucide-svelte';

	interface Props {
		isVisible: boolean;
		onDismiss: () => void;
		inattentiveSeconds: number;
	}

	let { isVisible, onDismiss, inattentiveSeconds }: Props = $props();

	// Audio Alarm Logic
	$effect(() => {
		if (isVisible) {
			const AudioContextClass = window.AudioContext || (window as any).webkitAudioContext;
			if (!AudioContextClass) return;

			const audioContext = new AudioContextClass();
			let interval: ReturnType<typeof setInterval>;

			const playAlarm = () => {
				// Prevent stacking sounds if context is suspended or closed
				if (audioContext.state === 'closed') return;

				const oscillator = audioContext.createOscillator();
				const gainNode = audioContext.createGain();

				oscillator.connect(gainNode);
				gainNode.connect(audioContext.destination);

				oscillator.frequency.value = 800; // Hz
				oscillator.type = 'square';

				// Ramp volume down quickly for a "beep" effect
				gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
				gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.5);

				oscillator.start(audioContext.currentTime);
				oscillator.stop(audioContext.currentTime + 0.5);
			};

			// Play immediately, then loop
			playAlarm();
			interval = setInterval(playAlarm, 1000);

			// Cleanup function
			return () => {
				clearInterval(interval);
				audioContext.close();
			};
		}
	});
</script>

{#if isVisible}
	<div class="backdrop" role="alertdialog" aria-modal="true" transition:fade={{ duration: 200 }}>
		<!-- Pulsing background layer -->
		<div class="pulse-overlay"></div>

		<div
			class="modal-card"
			in:scale={{ duration: 300, start: 0.8, easing: cubicOut }}
			out:scale={{ duration: 200, start: 0.8 }}
		>
			<!-- Icon -->
			<div class="icon-wrapper">
				<TriangleAlert size={80} />
			</div>

			<!-- Title -->
			<h2 class="alert-title">⚠ ATTENTION VIOLATION ⚠</h2>

			<!-- Message -->
			<p class="message-text">
				You have been inattentive for <span class="seconds-highlight"
					>{inattentiveSeconds} seconds</span
				>.
			</p>

			<p class="sub-message">
				Your viewing privileges may be revoked. Please return your attention to the advertisements
				immediately.
			</p>

			<!-- Sound Indicator -->
			<div class="sound-indicator">
				<div class="pulse-icon">
					<Volume2 size={16} />
				</div>
				<span>ALARM ACTIVE</span>
			</div>

			<!-- Dismiss Button -->
			<button class="dismiss-btn" onclick={onDismiss}> I Promise to Watch the Ads </button>

			<!-- Fine Print -->
			<p class="fine-print">
				By clicking this button, you agree to surrender your attention to our sponsors.
			</p>
		</div>
	</div>
{/if}

<style>
	/* --- Theme Variables --- */
	:global(:root) {
		--c-bg-overlay: rgba(2, 6, 23, 0.95); /* dark slate */
		--c-card-bg: #0f172a; /* slate 900 */
		--c-primary: #ef4444; /* red 500 */
		--c-primary-fg: #ffffff;
		--c-text: #f8fafc;
		--c-muted: #94a3b8;
		--font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
	}

	/* --- Layout --- */
	.backdrop {
		position: fixed;
		inset: 0;
		z-index: 50;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: var(--c-bg-overlay);
		backdrop-filter: blur(4px);
	}

	/* Pulsing Background Animation */
	.pulse-overlay {
		position: absolute;
		inset: 0;
		background-color: var(--c-primary);
		opacity: 0.1;
		animation: pulse-bg 1s infinite;
		pointer-events: none; /* Let clicks pass through to modal */
	}

	.modal-card {
		position: relative;
		z-index: 10;
		max-width: 32rem;
		margin: 0 1rem;
		padding: 2rem;
		background-color: var(--c-card-bg);
		border: 4px solid var(--c-primary);
		border-radius: 0.5rem;
		text-align: center;
		color: var(--c-text);

		/* Box Glow Red */
		box-shadow:
			0 0 20px rgba(239, 68, 68, 0.4),
			inset 0 0 20px rgba(239, 68, 68, 0.1);
	}

	/* --- Typography & Content --- */
	.icon-wrapper {
		margin-bottom: 1.5rem;
		color: var(--c-primary);
		display: inline-block;
		animation: heartbeat 1s infinite ease-in-out;
	}

	.alert-title {
		font-size: 1.875rem;
		font-weight: 900;
		color: var(--c-primary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 1rem;
		text-shadow: 0 0 10px rgba(239, 68, 68, 0.6);
		animation: flash-text 0.6s infinite;
	}

	.message-text {
		font-size: 1.125rem;
		margin-bottom: 1rem;
		color: var(--c-text);
	}

	.seconds-highlight {
		font-weight: 700;
		color: var(--c-primary);
	}

	.sub-message {
		color: var(--c-muted);
		margin-bottom: 1.5rem;
	}

	.sound-indicator {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		font-size: 0.875rem;
		color: var(--c-muted);
		font-family: var(--font-mono);
		margin-bottom: 1.5rem;
	}

	.pulse-icon {
		animation: pulse-opacity 1s infinite;
	}

	.fine-print {
		margin-top: 1rem;
		font-size: 0.75rem;
		color: var(--c-muted);
		font-family: var(--font-mono);
	}

	/* --- Button --- */
	.dismiss-btn {
		padding: 1rem 2rem;
		background-color: var(--c-primary);
		color: var(--c-primary-fg);
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-radius: 0.25rem;
		font-size: 1.125rem;
		border: none;
		cursor: pointer;
		transition:
			transform 0.1s,
			background-color 0.2s;
	}

	.dismiss-btn:hover {
		background-color: color-mix(in srgb, var(--c-primary), black 10%);
		transform: scale(1.05);
	}

	.dismiss-btn:active {
		transform: scale(0.95);
	}

	/* --- Keyframes (Replacing Framer Motion) --- */
	@keyframes pulse-bg {
		0%,
		100% {
			opacity: 0.1;
		}
		50% {
			opacity: 0.3;
		}
	}

	@keyframes heartbeat {
		0%,
		100% {
			transform: scale(1);
		}
		50% {
			transform: scale(1.1);
		}
	}

	@keyframes flash-text {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.7;
		}
	}

	@keyframes pulse-opacity {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0.5;
		}
	}
</style>
