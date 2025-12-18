// Add Guide component

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { X, Loader2, BookPlus, Check } from 'lucide-react';
import { useGuideStore } from '../stores/guideStore';

interface AddGuideProps {
  isOpen: boolean;
  onClose: () => void;
}

export function AddGuide({ isOpen, onClose }: AddGuideProps) {
  const { t } = useTranslation();
  const { createGuide, isLoading } = useGuideStore();

  const [input, setInput] = useState('');
  const [status, setStatus] = useState<'idle' | 'creating' | 'success' | 'error'>('idle');
  const [result, setResult] = useState<{ path: string; title: string } | null>(null);
  const [error, setError] = useState('');

  if (!isOpen) return null;

  const handleCreate = async () => {
    if (!input.trim() || isLoading) return;

    setStatus('creating');
    setError('');

    try {
      const created = await createGuide(input.trim());
      setResult(created);
      setStatus('success');
      setInput('');
    } catch (e) {
      setError(String(e));
      setStatus('error');
    }
  };

  const handleClose = () => {
    setStatus('idle');
    setResult(null);
    setError('');
    setInput('');
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-900 border border-gray-700 rounded-xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-gray-700">
          <div className="flex items-center gap-2">
            <BookPlus className="w-5 h-5 text-ai-glow" />
            <h2 className="text-lg font-semibold">{t('guide.add.title')}</h2>
          </div>
          <button
            onClick={handleClose}
            className="p-1 hover:bg-gray-800 rounded-lg transition-colors"
          >
            <X className="w-5 h-5 text-gray-400" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6">
          {status === 'success' && result ? (
            <div className="text-center py-8">
              <div className="inline-flex items-center justify-center w-12 h-12 rounded-full bg-green-500/20 mb-4">
                <Check className="w-6 h-6 text-green-400" />
              </div>
              <h3 className="text-lg font-medium mb-2">{t('guide.add.success')}</h3>
              <p className="text-gray-400 mb-1">{result.title}</p>
              <p className="text-gray-500 text-sm">{result.path}</p>
              <button
                onClick={handleClose}
                className="mt-6 px-4 py-2 bg-ai-glow hover:bg-ai-glow/80 rounded-lg transition-colors"
              >
                {t('common.close')}
              </button>
            </div>
          ) : (
            <>
              <p className="text-gray-400 mb-4">{t('guide.add.description')}</p>

              <textarea
                value={input}
                onChange={(e) => setInput(e.target.value)}
                placeholder={t('guide.add.placeholder')}
                disabled={status === 'creating'}
                rows={6}
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-3
                         text-white placeholder-gray-500 focus:outline-none focus:border-ai-glow
                         focus:ring-1 focus:ring-ai-glow transition-colors resize-none
                         disabled:opacity-50 disabled:cursor-not-allowed"
              />

              {status === 'error' && (
                <p className="mt-2 text-sm text-red-400">{error}</p>
              )}

              <div className="flex justify-end gap-3 mt-4">
                <button
                  onClick={handleClose}
                  className="px-4 py-2 text-gray-400 hover:text-white transition-colors"
                >
                  {t('common.cancel')}
                </button>
                <button
                  onClick={handleCreate}
                  disabled={!input.trim() || status === 'creating'}
                  className="px-4 py-2 bg-ai-glow hover:bg-ai-glow/80 rounded-lg transition-colors
                           disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                >
                  {status === 'creating' ? (
                    <>
                      <Loader2 className="w-4 h-4 animate-spin" />
                      {t('guide.add.creating')}
                    </>
                  ) : (
                    t('guide.add.create')
                  )}
                </button>
              </div>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
