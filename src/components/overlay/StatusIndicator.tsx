// Status indicator overlay component

import { motion, AnimatePresence } from 'motion/react';
import { useOverlayStore, OverlayStatus } from '../../stores/overlayStore';
import { useShallow } from 'zustand/shallow';
import { Bot, MousePointer2, MousePointerClick, Keyboard, ScrollText, Brain } from 'lucide-react';

const statusConfig: Record<OverlayStatus, { icon: React.ElementType; label: string; color: string }> = {
  idle: { icon: Bot, label: 'Ready', color: 'bg-gray-600' },
  thinking: { icon: Brain, label: 'Thinking...', color: 'bg-purple-600' },
  moving: { icon: MousePointer2, label: 'Moving', color: 'bg-blue-600' },
  clicking: { icon: MousePointerClick, label: 'Clicking', color: 'bg-indigo-600' },
  typing: { icon: Keyboard, label: 'Typing', color: 'bg-green-600' },
  scrolling: { icon: ScrollText, label: 'Scrolling', color: 'bg-cyan-600' },
};

export function StatusIndicator() {
  const { status, statusMessage, isAiControlling } = useOverlayStore(
    useShallow((state) => ({
      status: state.status,
      statusMessage: state.statusMessage,
      isAiControlling: state.isAiControlling,
    }))
  );

  const config = statusConfig[status];
  const Icon = config.icon;

  return (
    <AnimatePresence>
      {isAiControlling && status !== 'idle' && (
        <motion.div
          className="fixed bottom-8 left-1/2 pointer-events-none"
          initial={{ y: 100, x: '-50%', opacity: 0 }}
          animate={{ y: 0, x: '-50%', opacity: 1 }}
          exit={{ y: 100, x: '-50%', opacity: 0 }}
          transition={{ type: 'spring', damping: 20, stiffness: 300 }}
        >
          <div
            className={`${config.color} rounded-full px-6 py-3 flex items-center gap-3 shadow-2xl`}
            style={{
              backdropFilter: 'blur(10px)',
              boxShadow: '0 4px 30px rgba(0, 0, 0, 0.3)',
            }}
          >
            {/* Animated icon */}
            <motion.div
              animate={{
                scale: [1, 1.1, 1],
              }}
              transition={{
                duration: 1,
                repeat: Infinity,
                ease: 'easeInOut',
              }}
            >
              <Icon className="w-5 h-5 text-white" />
            </motion.div>

            {/* Status text */}
            <div className="flex flex-col">
              <span className="text-white font-medium text-sm">{config.label}</span>
              {statusMessage && (
                <span className="text-white/70 text-xs">{statusMessage}</span>
              )}
            </div>

            {/* Pulse indicator */}
            <motion.div
              className="w-2 h-2 rounded-full bg-white"
              animate={{
                opacity: [1, 0.3, 1],
                scale: [1, 0.8, 1],
              }}
              transition={{
                duration: 1,
                repeat: Infinity,
                ease: 'easeInOut',
              }}
            />
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
