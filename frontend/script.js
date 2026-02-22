/* ═══════════════════════════════════════════════════
   LATTICEPRESS — INTERACTIONS & ANIMATIONS
   ═══════════════════════════════════════════════════ */

document.addEventListener('DOMContentLoaded', () => {

    // ── Mobile Nav Toggle ────────────────────────────
    const navToggle = document.getElementById('navToggle');
    const navLinks = document.getElementById('navLinks');

    if (navToggle) {
        navToggle.addEventListener('click', () => {
            navLinks.classList.toggle('open');
        });

        // Close nav when clicking a link
        navLinks.querySelectorAll('a').forEach(link => {
            link.addEventListener('click', () => {
                navLinks.classList.remove('open');
            });
        });
    }

    // ── Animated Counters ────────────────────────────
    function animateCounter(el) {
        const target = parseFloat(el.dataset.target);
        const isDecimal = String(target).includes('.');
        const decimals = isDecimal ? String(target).split('.')[1].length : 0;
        const duration = 1800;
        const startTime = performance.now();

        function tick(now) {
            const elapsed = now - startTime;
            const progress = Math.min(elapsed / duration, 1);
            // Ease out cubic
            const ease = 1 - Math.pow(1 - progress, 3);
            const current = target * ease;

            el.textContent = isDecimal ? current.toFixed(decimals) : Math.floor(current);

            if (progress < 1) {
                requestAnimationFrame(tick);
            } else {
                el.textContent = isDecimal ? target.toFixed(decimals) : target;
            }
        }

        requestAnimationFrame(tick);
    }

    // ── Intersection Observer (scroll animations) ────
    const observerOptions = {
        threshold: 0.2,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                // Animate-in elements
                if (entry.target.classList.contains('animate-in')) {
                    entry.target.classList.add('visible');
                }

                // Counters
                if (entry.target.classList.contains('counter') && !entry.target.dataset.animated) {
                    entry.target.dataset.animated = 'true';
                    animateCounter(entry.target);
                }

                // Bar charts
                if (entry.target.classList.contains('bar-group')) {
                    const bars = entry.target.querySelectorAll('.bar');
                    bars.forEach((bar, i) => {
                        setTimeout(() => {
                            bar.style.width = bar.dataset.width + '%';
                        }, i * 200);
                    });
                }
            }
        });
    }, observerOptions);

    // Observe all animated elements
    document.querySelectorAll('.animate-in').forEach(el => observer.observe(el));
    document.querySelectorAll('.counter').forEach(el => observer.observe(el));
    document.querySelectorAll('.bar-group').forEach(el => observer.observe(el));

    // ── Live Counter Simulation ──────────────────────
    const bytesSavedEl = document.getElementById('bytesSaved');
    const txCompressedEl = document.getElementById('txCompressed');
    const storageReclaimedEl = document.getElementById('storageReclaimed');

    if (bytesSavedEl && txCompressedEl && storageReclaimedEl) {
        // 10,000 TPS, each saving 375 bytes (500 - 125)
        const BYTES_PER_TX = 375;
        const TPS = 10000;
        const UPDATE_INTERVAL = 50; // ms
        const TX_PER_TICK = (TPS * UPDATE_INTERVAL) / 1000;

        let totalBytes = 0;
        let totalTx = 0;

        function formatBytes(bytes) {
            if (bytes >= 1_000_000_000) {
                return (bytes / 1_000_000_000).toFixed(2) + ' GB';
            } else if (bytes >= 1_000_000) {
                return (bytes / 1_000_000).toFixed(2) + ' MB';
            } else if (bytes >= 1_000) {
                return (bytes / 1_000).toFixed(1) + ' KB';
            }
            return bytes + ' B';
        }

        function formatNumber(num) {
            return num.toLocaleString('en-US');
        }

        // Only start when the live section is visible
        let liveStarted = false;
        const liveSection = document.getElementById('live');

        const liveObserver = new IntersectionObserver((entries) => {
            if (entries[0].isIntersecting && !liveStarted) {
                liveStarted = true;
                setInterval(() => {
                    totalTx += TX_PER_TICK;
                    totalBytes += TX_PER_TICK * BYTES_PER_TX;

                    bytesSavedEl.textContent = formatNumber(Math.floor(totalBytes));
                    txCompressedEl.textContent = formatNumber(Math.floor(totalTx));
                    storageReclaimedEl.textContent = formatBytes(totalBytes);
                }, UPDATE_INTERVAL);
            }
        }, { threshold: 0.3 });

        liveObserver.observe(liveSection);
    }

    // ── Smooth scroll for anchor links ───────────────
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                e.preventDefault();
                target.scrollIntoView({ behavior: 'smooth', block: 'start' });
            }
        });
    });

    // ── Live Interceptor Demo ────────────────────────────
    const btnIntercept = document.getElementById('btnIntercept');
    if (btnIntercept) {
        btnIntercept.addEventListener('click', async () => {
            const payload = document.getElementById('demoPayload').value.trim();
            const statusEl = document.getElementById('demoStatus');
            statusEl.textContent = 'Intercepting & Sending...';
            
            try {
                const res = await fetch('http://localhost:3000/intercept', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ payload_hex: payload })
                });
                
                if (res.ok) {
                    const data = await res.json();
                    document.getElementById('liveOriginal').textContent = data.original_size;
                    document.getElementById('livePacked').textContent = data.packed_size;
                    document.getElementById('liveGas').textContent = data.gas_used;
                    document.getElementById('liveTime').textContent = data.compression_time_ms.toFixed(4);
                    
                    const hashLink = document.getElementById('liveHash');
                    hashLink.textContent = data.tx_hash.substring(0,10) + '...';
                    
                    statusEl.textContent = 'Success!';
                } else {
                    statusEl.textContent = 'Error: API failed';
                }
            } catch (err) {
                statusEl.textContent = 'Error: Is sidecar running?';
                console.error(err);
            }
        });
    }
});
