// Toast notification component

import { motion, AnimatePresence } from 'motion/react';
import { X, CheckCircle, XCircle, AlertTriangle, Info } from 'lucide-react';
import { useToastStore, Toast as ToastType, ToastType as TType } from '../stores/toastStore';
import { useShallow } from 'zustand/shallow';

const icons: Record<TType, React.ElementType> = {
  success: CheckCircle,
  error: XCircle,
  warning: AlertTriangle,
  info: Info,
};

const colors: Record<TType, string> = {
  success: 'bg-green-600',
  error: 'bg-red-600',
  warning: 'bg-yellow-600',
  info: 'bg-blue-600',
};

export function ToastContainer() {
  const { toasts, removeToast } = useToastStore(
    useShallow((state) => ({
      toasts: state.toasts,
      removeToast: state.removeToast,
    }))
  );

  return (
    <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
      <AnimatePresence>
        {toasts.map((toast) => (
          <ToastItem key={toast.id} toast={toast} onClose={() => removeToast(toast.id)} />
        ))}
      </AnimatePresence>
    </div>
  );
}

function ToastItem({ toast, onClose }: { toast: ToastType; onClose: () => void }) {
  const Icon = icons[toast.type];
  const color = colors[toast.type];

  return (
    <motion.div
      initial={{ opacity: 0, x: 100, scale: 0.9 }}
      animate={{ opacity: 1, x: 0, scale: 1 }}
      exit={{ opacity: 0, x: 100, scale: 0.9 }}
      className={`${color} rounded-lg shadow-lg px-4 py-3 flex items-center gap-3 min-w-[300px] max-w-[400px]`}
    >
      <Icon className="w-5 h-5 text-white shrink-0" />
      <p className="text-white text-sm flex-1">{toast.message}</p>
      <button
        onClick={onClose}
        className="text-white/70 hover:text-white transition-colors shrink-0"
      >
        <X className="w-4 h-4" />
      </button>
    </motion.div>
  );
}
