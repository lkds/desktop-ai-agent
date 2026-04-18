import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api'
import { Package, Download, Trash2 } from 'lucide-react'

interface Skill {
  id: string
  name: string
  description: string
  triggers: string[]
  installed: boolean
}

function SkillsPage() {
  const [skills, setSkills] = useState<Skill[]>([])
  const [loading, setLoading] = useState(true)
  
  useEffect(() => {
    loadSkills()
  }, [])
  
  const loadSkills = async () => {
    try {
      const installedSkills = await invoke<Skill[]>('list_skills')
      setSkills(installedSkills.map(s => ({ ...s, installed: true })))
    } catch (error) {
      console.error('Load skills failed:', error)
    }
    setLoading(false)
  }
  
  const installSkill = async (skillId: string) => {
    try {
      await invoke('install_skill', { skillId })
      loadSkills()
    } catch (error) {
      console.error('Install failed:', error)
    }
  }
  
  const uninstallSkill = async (skillId: string) => {
    try {
      await invoke('uninstall_skill', { skillId })
      loadSkills()
    } catch (error) {
      console.error('Uninstall failed:', error)
    }
  }
  
  if (loading) {
    return <div style={{ padding: '20px' }}>加载中...</div>
  }
  
  return (
    <div className="skills-container">
      <h2 style={{ marginBottom: '20px' }}>技能管理</h2>
      
      <div className="skills-grid">
        {skills.map(skill => (
          <div key={skill.id} className="skill-card">
            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
              <Package size={16} />
              <h3>{skill.name}</h3>
            </div>
            <p>{skill.description}</p>
            
            {skill.triggers.length > 0 && (
              <div style={{ marginTop: '8px', fontSize: '12px', color: 'var(--text-muted)' }}>
                触发词: {skill.triggers.join(', ')}
              </div>
            )}
            
            <div style={{ marginTop: '12px' }}>
              {skill.installed ? (
                <button
                  onClick={() => uninstallSkill(skill.id)}
                  style={{
                    background: 'var(--error)',
                    color: 'white',
                    border: 'none',
                    borderRadius: '6px',
                    padding: '6px 12px',
                    cursor: 'pointer',
                    fontSize: '12px'
                  }}
                >
                  <Trash2 size={12} />
                  卸载
                </button>
              ) : (
                <button
                  onClick={() => installSkill(skill.id)}
                  style={{
                    background: 'var(--primary)',
                    color: 'white',
                    border: 'none',
                    borderRadius: '6px',
                    padding: '6px 12px',
                    cursor: 'pointer',
                    fontSize: '12px'
                  }}
                >
                  <Download size={12} />
                  安装
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
      
      {skills.length === 0 && (
        <div style={{ color: 'var(--text-muted)' }}>
          暂无已安装的技能
        </div>
      )}
    </div>
  )
}

export default SkillsPage