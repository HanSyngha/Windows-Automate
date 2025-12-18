import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Bot, Settings as SettingsIcon, BookPlus } from 'lucide-react';
import { Chat } from './components/Chat';
import { Settings } from './components/Settings';
import { AddGuide } from './components/AddGuide';
import { ToastContainer } from './components/Toast';
import { UpdateChecker } from './components/UpdateChecker';
import { useConfigStore } from './stores/configStore';

function App() {
  const { t, i18n } = useTranslation();
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [isAddGuideOpen, setIsAddGuideOpen] = useState(false);
  const { config, loadConfig } = useConfigStore();

  // Load config on mount
  useEffect(() => {
    loadConfig();
  }, [loadConfig]);

  // Update language when config changes
  useEffect(() => {
    if (config.language && config.language !== i18n.language) {
      i18n.changeLanguage(config.language);
    }
  }, [config.language, i18n]);

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Header */}
      <header className="border-b border-gray-800 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-ai-glow/20 rounded-lg">
              <Bot className="w-6 h-6 text-ai-glow" />
            </div>
            <h1 className="text-xl font-semibold">AutoMate</h1>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={() => setIsAddGuideOpen(true)}
              className="p-2 hover:bg-gray-800 rounded-lg transition-colors"
              title={t('guide.button')}
            >
              <BookPlus className="w-5 h-5 text-gray-400" />
            </button>
            <button
              onClick={() => setIsSettingsOpen(true)}
              className="p-2 hover:bg-gray-800 rounded-lg transition-colors"
              title={t('settings.title')}
            >
              <SettingsIcon className="w-5 h-5 text-gray-400" />
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="h-[calc(100vh-73px)]">
        <Chat />
      </main>

      {/* Settings Modal */}
      <Settings isOpen={isSettingsOpen} onClose={() => setIsSettingsOpen(false)} />

      {/* Add Guide Modal */}
      <AddGuide isOpen={isAddGuideOpen} onClose={() => setIsAddGuideOpen(false)} />

      {/* Toast Notifications */}
      <ToastContainer />

      {/* Update Checker */}
      <UpdateChecker />
    </div>
  );
}

export default App;
