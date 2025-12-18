// Click ripple effect component

import { motion, AnimatePresence } from 'motion/react';
import { useOverlayStore, ClickEffect } from '../../stores/overlayStore';
import { useShallow } from 'zustand/shallow';

export function ClickRipple() {
  const clickEffects = useOverlayStore(useShallow((state) => state.clickEffects));

  return (
    <AnimatePresence>
      {clickEffects.map((effect) => (
        <RippleEffect key={effect.id} effect={effect} />
      ))}
    </AnimatePresence>
  );
}

function RippleEffect({ effect }: { effect: ClickEffect }) {
  const getColor = () => {
    switch (effect.button) {
      case 'right':
        return 'rgba(239, 68, 68, 0.6)'; // red
      case 'middle':
        return 'rgba(234, 179, 8, 0.6)'; // yellow
      default:
        return 'rgba(99, 102, 241, 0.6)'; // indigo
    }
  };

  const color = getColor();

  return (
    <motion.div
      className="fixed pointer-events-none"
      style={{
        left: effect.x,
        top: effect.y,
        transform: 'translate(-50%, -50%)',
      }}
      initial={{ opacity: 1 }}
      animate={{ opacity: 0 }}
      exit={{ opacity: 0 }}
      transition={{ duration: 0.6 }}
    >
      {/* Outer ripple */}
      <motion.div
        className="absolute rounded-full"
        style={{
          border: `2px solid ${color}`,
        }}
        initial={{ width: 10, height: 10, x: -5, y: -5, opacity: 1 }}
        animate={{ width: 80, height: 80, x: -40, y: -40, opacity: 0 }}
        transition={{ duration: 0.5, ease: 'easeOut' }}
      />

      {/* Middle ripple */}
      <motion.div
        className="absolute rounded-full"
        style={{
          border: `2px solid ${color}`,
        }}
        initial={{ width: 10, height: 10, x: -5, y: -5, opacity: 1 }}
        animate={{ width: 50, height: 50, x: -25, y: -25, opacity: 0 }}
        transition={{ duration: 0.4, ease: 'easeOut', delay: 0.05 }}
      />

      {/* Center dot */}
      <motion.div
        className="absolute rounded-full"
        style={{
          backgroundColor: color,
          boxShadow: `0 0 10px 2px ${color}`,
        }}
        initial={{ width: 12, height: 12, x: -6, y: -6, scale: 1 }}
        animate={{ scale: 0 }}
        transition={{ duration: 0.3, ease: 'easeOut', delay: 0.1 }}
      />
    </motion.div>
  );
}
