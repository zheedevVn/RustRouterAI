pub const MAIN_CSS: &str = r#"
* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    -webkit-tap-highlight-color: transparent;
}

body, html {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    background-color: #1e1f22;
    color: #dbdee1;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    font-size: 14px;
    line-height: 1.5;
    overflow: hidden;
    position: fixed;
    inset: 0;
    user-select: none;
    -webkit-user-select: none;
    -webkit-touch-callout: none;
}

#main {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
}

/* Safe areas for iOS iPhone notch and home indicator */
.app-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    flex: 1;
    padding-top: env(safe-area-inset-top, 0px);
    padding-bottom: env(safe-area-inset-bottom, 0px);
    padding-left: env(safe-area-inset-left, 0px);
    padding-right: env(safe-area-inset-right, 0px);
    background-color: #1e1f22;
    overflow: hidden;
    box-sizing: border-box;
}

/* Header Bar */
.top-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 48px;
    padding: 0 16px;
    background: #2b2d31;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    flex-shrink: 0;
    z-index: 100;
}

.brand-badge {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
    font-weight: 700;
    color: #ffffff;
}

.brand-badge img {
    height: 28px;
    object-fit: contain;
}

.status-pill {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 3px 10px;
    background: rgba(82, 196, 26, 0.15);
    border: 1px solid rgba(82, 196, 26, 0.3);
    border-radius: 12px;
    color: #52c41a;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
}

/* Main Viewport Content */
.main-viewport {
    display: flex;
    flex: 1;
    overflow: hidden;
    position: relative;
}

.content-scroll {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 16px;
    -webkit-overflow-scrolling: touch;
}

/* Bottom Navigation Bar for Mobile */
.bottom-nav {
    display: flex;
    align-items: center;
    justify-content: space-around;
    height: 56px;
    background: #2b2d31;
    border-top: 1px solid rgba(255, 255, 255, 0.08);
    flex-shrink: 0;
    z-index: 100;
}

.nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 3px;
    flex: 1;
    height: 100%;
    color: #949ba4;
    text-decoration: none;
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.nav-item.active {
    color: #1890ff;
}

.nav-item svg {
    width: 20px;
    height: 20px;
    stroke: currentColor;
    fill: none;
    stroke-width: 1.8;
}

/* Cards & Widgets */
.card {
    background: #2b2d31;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 10px;
    padding: 16px;
    margin-bottom: 14px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.card-title {
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
    margin-bottom: 12px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

/* Metric Stats Grid */
.stats-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
    margin-bottom: 16px;
}

.stat-box {
    background: #313338;
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 8px;
    padding: 12px;
}

.stat-label {
    font-size: 11px;
    color: #949ba4;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 4px;
}

.stat-value {
    font-size: 20px;
    font-weight: 700;
    color: #ffffff;
}

/* Buttons */
.btn-primary {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 100%;
    height: 42px;
    background: #1890ff;
    color: #ffffff;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    box-shadow: 0 2px 8px rgba(24, 144, 255, 0.3);
    transition: background 0.2s ease;
}

.btn-primary:active {
    background: #40a9ff;
}

.btn-secondary {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    width: 100%;
    height: 40px;
    background: #313338;
    color: #dbdee1;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.btn-secondary:active {
    background: #383a40;
}

/* Inputs */
.input-field {
    width: 100%;
    height: 40px;
    background: #1e1f22;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 6px;
    color: #ffffff;
    padding: 0 12px;
    font-size: 14px;
    outline: none;
    margin-bottom: 12px;
    transition: border-color 0.2s ease;
}

.input-field:focus {
    border-color: #1890ff;
}

/* Modals */
.modal-overlay {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.65);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    z-index: 1000;
}

.modal-card {
    width: 100%;
    max-width: 400px;
    background: #313338;
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 8px 32px rgba(0,0,0,0.5);
}
"#;
