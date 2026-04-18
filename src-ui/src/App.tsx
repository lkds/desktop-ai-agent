import { useState } from 'react'
import { useChatStore } from './stores/chatStore'
import { useConfigStore } from './stores/configStore'
import ChatPage from './pages/ChatPage'
import SettingsPage from './pages/SettingsPage'
import SkillsPage from './pages/SkillsPage'
import HistoryPage from './pages/HistoryPage'
import { MessageSquare, Settings, Package, History } from 'lucide-react'

function App() {
  const [currentPage, setCurrentPage] = useState<'chat' | 'settings' | 'skills' | 'history'>('chat')
  
  return (
    <div className="app-container">
      {/* Sidebar */}
      <nav className="sidebar">
        <div className="sidebar-header">
          <h1>Desktop Agent</h1>
        </div>
        
        <div className="sidebar-nav">
          <button 
            className={`nav-item ${currentPage === 'chat' ? 'active' : ''}`}
            onClick={() => setCurrentPage('chat')}
          >
            <MessageSquare size={20} />
            <span>对话</span>
          </button>
          
          <button 
            className={`nav-item ${currentPage === 'settings' ? 'active' : ''}`}
            onClick={() => setCurrentPage('settings')}
          >
            <Settings size={20} />
            <span>设置</span>
          </button>
          
          <button 
            className={`nav-item ${currentPage === 'skills' ? 'active' : ''}`}
            onClick={() => setCurrentPage('skills')}
          >
            <Package size={20} />
            <span>技能</span>
          </button>
          
          <button 
            className={`nav-item ${currentPage === 'history' ? 'active' : ''}`}
            onClick={() => setCurrentPage('history')}
          >
            <History size={20} />
            <span>历史</span>
          </button>
        </div>
        
        <div className="sidebar-footer">
          <div className="model-status">
            当前模型: {useConfigStore.getState().currentModel || '未配置'}
          </div>
        </div>
      </nav>
      
      {/* Main Content */}
      <main className="main-content">
        {currentPage === 'chat' && <ChatPage />}
        {currentPage === 'settings' && <SettingsPage />}
        {currentPage === 'skills' && <SkillsPage />}
        {currentPage === 'history' && <HistoryPage />}
      </main>
    </div>
  )
}

export default App