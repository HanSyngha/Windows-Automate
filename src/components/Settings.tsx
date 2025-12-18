// Settings component

import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { X, Check, Loader2, Eye, EyeOff } from 'lucide-react';
import { useConfigStore } from '../stores/configStore';

interface SettingsProps {
  isOpen: boolean;
  onClose: () => void;
}

export function Settings({ isOpen, onClose }: SettingsProps) {
  const { t } = useTranslation();
  const { config, loadConfig, saveConfig, testConnection, isLoading } = useConfigStore();

  const [localConfig, setLocalConfig] = useState(config);
  const [showApiKey, setShowApiKey] = useState(false);
  const [testStatus, setTestStatus] = useState<'idle' | 'testing' | 'success' | 'error'>('idle');
  const [testMessage, setTestMessage] = useState('');

  useEffect(() => {
    if (isOpen) {
      loadConfig();
    }
  }, [isOpen, loadConfig]);

  useEffect(() => {
    setLocalConfig(config);
  }, [config]);

  if (!isOpen) return null;

  const handleSave = async () => {
    await saveConfig(localConfig);
    onClose();
  };

  const handleTest = async () => {
    setTestStatus('testing');
    try {
      const result = await testConnection(localConfig.api);
      setTestStatus('success');
      setTestMessage(result);
    } catch (e) {
      setTestStatus('error');
      setTestMessage(String(e));
    }
  };

  const updateApi = (key: string, value: string | number | boolean) => {
    setLocalConfig({
      ...localConfig,
      api: { ...localConfig.api, [key]: value },
    });
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-gray-900 border border-gray-700 rounded-xl w-full max-w-lg mx-4 max-h-[90vh] overflow-y-auto">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-gray-700">
          <h2 className="text-lg font-semibold">{t('settings.title')}</h2>
          <button
            onClick={onClose}
            className="p-1 hover:bg-gray-800 rounded-lg transition-colors"
          >
            <X className="w-5 h-5 text-gray-400" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6">
          {/* API Settings */}
          <section>
            <h3 className="text-sm font-medium text-gray-400 mb-4">{t('settings.api.title')}</h3>

            {/* Endpoint */}
            <div className="mb-4">
              <label className="block text-sm mb-2">{t('settings.api.endpoint')}</label>
              <input
                type="text"
                value={localConfig.api.endpoint}
                onChange={(e) => updateApi('endpoint', e.target.value)}
                placeholder="https://api.openai.com/v1"
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2
                         text-white placeholder-gray-500 focus:outline-none focus:border-ai-glow"
              />
            </div>

            {/* API Key */}
            <div className="mb-4">
              <label className="block text-sm mb-2">{t('settings.api.apiKey')}</label>
              <div className="relative">
                <input
                  type={showApiKey ? 'text' : 'password'}
                  value={localConfig.api.api_key}
                  onChange={(e) => updateApi('api_key', e.target.value)}
                  placeholder="sk-..."
                  className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2 pr-10
                           text-white placeholder-gray-500 focus:outline-none focus:border-ai-glow"
                />
                <button
                  type="button"
                  onClick={() => setShowApiKey(!showApiKey)}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-white"
                >
                  {showApiKey ? <EyeOff className="w-4 h-4" /> : <Eye className="w-4 h-4" />}
                </button>
              </div>
            </div>

            {/* Model */}
            <div className="mb-4">
              <label className="block text-sm mb-2">{t('settings.api.model')}</label>
              <input
                type="text"
                value={localConfig.api.model}
                onChange={(e) => updateApi('model', e.target.value)}
                placeholder="gpt-4o"
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2
                         text-white placeholder-gray-500 focus:outline-none focus:border-ai-glow"
              />
            </div>

            {/* Vision Support */}
            <div className="mb-4 flex items-center gap-3">
              <input
                type="checkbox"
                id="supports_vision"
                checked={localConfig.api.supports_vision}
                onChange={(e) => updateApi('supports_vision', e.target.checked)}
                className="w-4 h-4 rounded border-gray-600 bg-gray-800 text-ai-glow focus:ring-ai-glow"
              />
              <label htmlFor="supports_vision" className="text-sm">
                {t('settings.api.supportsVision')}
              </label>
            </div>

            {/* Max Tokens */}
            <div className="mb-4">
              <label className="block text-sm mb-2">{t('settings.api.maxTokens')}</label>
              <input
                type="number"
                value={localConfig.api.max_tokens}
                onChange={(e) => updateApi('max_tokens', parseInt(e.target.value) || 4096)}
                min={1}
                max={128000}
                className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2
                         text-white placeholder-gray-500 focus:outline-none focus:border-ai-glow"
              />
            </div>

            {/* Temperature */}
            <div className="mb-4">
              <label className="block text-sm mb-2">
                {t('settings.api.temperature')}: {localConfig.api.temperature}
              </label>
              <input
                type="range"
                value={localConfig.api.temperature}
                onChange={(e) => updateApi('temperature', parseFloat(e.target.value))}
                min={0}
                max={2}
                step={0.1}
                className="w-full accent-ai-glow"
              />
            </div>

            {/* Test Connection */}
            <button
              onClick={handleTest}
              disabled={testStatus === 'testing' || !localConfig.api.api_key}
              className="w-full py-2 px-4 bg-gray-800 hover:bg-gray-700 border border-gray-600
                       rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed
                       flex items-center justify-center gap-2"
            >
              {testStatus === 'testing' ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  {t('settings.api.testing')}
                </>
              ) : (
                t('settings.api.testConnection')
              )}
            </button>

            {testStatus === 'success' && (
              <p className="mt-2 text-sm text-green-400 flex items-center gap-1">
                <Check className="w-4 h-4" />
                {testMessage}
              </p>
            )}

            {testStatus === 'error' && (
              <p className="mt-2 text-sm text-red-400">{testMessage}</p>
            )}
          </section>

          {/* Language */}
          <section>
            <h3 className="text-sm font-medium text-gray-400 mb-4">{t('settings.language.title')}</h3>
            <select
              value={localConfig.language}
              onChange={(e) => setLocalConfig({ ...localConfig, language: e.target.value })}
              className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2
                       text-white focus:outline-none focus:border-ai-glow"
            >
              <option value="ko">한국어</option>
              <option value="en">English</option>
            </select>
          </section>

          {/* Shortcut */}
          <section>
            <h3 className="text-sm font-medium text-gray-400 mb-4">{t('settings.shortcut.title')}</h3>
            <input
              type="text"
              value={localConfig.global_shortcut}
              onChange={(e) => setLocalConfig({ ...localConfig, global_shortcut: e.target.value })}
              placeholder="Shift+Alt+A"
              className="w-full bg-gray-800 border border-gray-700 rounded-lg px-4 py-2
                       text-white placeholder-gray-500 focus:outline-none focus:border-ai-glow"
            />
          </section>
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-3 px-6 py-4 border-t border-gray-700">
          <button
            onClick={onClose}
            className="px-4 py-2 text-gray-400 hover:text-white transition-colors"
          >
            {t('common.cancel')}
          </button>
          <button
            onClick={handleSave}
            disabled={isLoading}
            className="px-4 py-2 bg-ai-glow hover:bg-ai-glow/80 rounded-lg transition-colors
                     disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
          >
            {isLoading && <Loader2 className="w-4 h-4 animate-spin" />}
            {t('common.save')}
          </button>
        </div>
      </div>
    </div>
  );
}
