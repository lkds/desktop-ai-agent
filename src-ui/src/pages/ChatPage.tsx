import { useState } from 'react'
import { useChatStore } from '../stores/chatStore'
import { Send, Square, X } from 'lucide-react'

function ChatPage() {
  const [input, setInput] = useState('')
  const { messages, currentTask, isExecuting, executeTask, pauseTask, cancelTask } = useChatStore()
  
  const handleSubmit = async () => {
    if (!input.trim() || isExecuting) return
    
    await executeTask(input.trim())
    setInput('')
  }
  
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSubmit()
    }
  }
  
  return (
    <div className="chat-container">
      {/* Messages */}
      <div className="chat-messages">
        {messages.length === 0 && (
          <div className="message system">
            输入任务描述，Agent 将自动执行并交付结果。
          </div>
        )}
        
        {messages.map(msg => (
          <div key={msg.id} className={`message ${msg.role}`}>
            {msg.content}
          </div>
        ))}
        
        {/* Task Steps */}
        {currentTask && currentTask.steps.length > 0 && (
          <div className="steps-container">
            <div className="message system">执行步骤：</div>
            {currentTask.steps.map(step => (
              <div key={step.id} className={`step-item ${step.status}`}>
                <div className={`step-status ${step.status}`}>
                  {step.status === 'running' && '●'}
                  {step.status === 'success' && '✓'}
                  {step.status === 'failed' && '✗'}
                </div>
                <span>{step.description}</span>
              </div>
            ))}
          </div>
        )}
      </div>
      
      {/* Input */}
      <div className="chat-input-container">
        <div className="chat-input">
          <textarea
            value={input}
            onChange={e => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="描述你想完成的任务..."
            disabled={isExecuting}
            rows={3}
          />
          
          {isExecuting ? (
            <div style={{ display: 'flex', gap: '8px' }}>
              <button onClick={pauseTask}>
                <Square size={16} />
                暂停
              </button>
              <button onClick={cancelTask} style={{ background: 'var(--error)' }}>
                <X size={16} />
                取消
              </button>
            </div>
          ) : (
            <button onClick={handleSubmit} disabled={!input.trim()}>
              <Send size={16} />
              发送
            </button>
          )}
        </div>
      </div>
    </div>
  )
}

export default ChatPage