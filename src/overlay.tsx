// Overlay window entry point

import { createRoot } from 'react-dom/client';
import { OverlayApp } from './components/overlay/OverlayApp';
import './overlay.css';

const container = document.getElementById('root');
if (container) {
  const root = createRoot(container);
  root.render(<OverlayApp />);
}
