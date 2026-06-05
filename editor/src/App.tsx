import { useState, useRef, useEffect } from 'react'
import './App.css'

type Effect = {
  tipo: string
  intensidade: number
  velocidade: number
  seed?: number
}

type ElementConfig = {
  id: string
  tag: string
  animacao?: Effect
}

const EFEITOS = ['organica', 'pulsar', 'deriva', 'blur', 'aparecer', 'impulso']

const EFEITO_COR: Record<string, string> = {
  organica: '#4ade80',
  pulsar:   '#f472b6',
  deriva:   '#60a5fa',
  blur:     '#a78bfa',
  aparecer: '#fbbf24',
  impulso:  '#fb923c',
}

const activeAnimations: Map<string, number> = new Map()

function stopAnimation(id: string) {
  const raf = activeAnimations.get(id)
  if (raf) cancelAnimationFrame(raf)
  activeAnimations.delete(id)
}

function stopAllAnimations() {
  activeAnimations.forEach((raf) => cancelAnimationFrame(raf))
  activeAnimations.clear()
}

function applyPreview(el: SVGElement, effect: Effect) {
  const id = el.id
  stopAnimation(id)
  el.style.filter = ''
  el.style.opacity = ''
  const existingTransform = el.getAttribute('data-original-transform') || el.getAttribute('transform') || ''
  if (!el.getAttribute('data-original-transform')) {
    el.setAttribute('data-original-transform', existingTransform)
  }
  const dur = effect.velocidade * 1000
  const amp = effect.intensidade
  const offset = Math.random() * dur

  if (effect.tipo === 'pulsar') {
    const rx0 = parseFloat(el.getAttribute('rx') || el.getAttribute('r') || '0')
    const ry0 = parseFloat(el.getAttribute('ry') || el.getAttribute('r') || '0')
    let start: number | null = null
    const phaseOffset = Math.random() * Math.PI * 2
    function tickPulsar(ts: number) {
      if (!start) start = ts
      const phase = ((ts - start) / 1000 / effect.velocidade) * Math.PI * 2 + phaseOffset
      const delta = Math.sin(phase) * amp * 0.5
      if (el.hasAttribute('rx')) {
        el.setAttribute('rx', String(rx0 + delta))
        el.setAttribute('ry', String(ry0 - delta * 0.7))
      } else if (el.hasAttribute('r')) {
        el.setAttribute('r', String(rx0 + delta))
      }
      activeAnimations.set(id, requestAnimationFrame(tickPulsar))
    }
    activeAnimations.set(id, requestAnimationFrame(tickPulsar))
  }

  else if (effect.tipo === 'deriva') {
    const base = el.getAttribute('data-original-transform') || ''
    const ox = Math.random() * Math.PI * 2
    const oy = Math.random() * Math.PI * 2
    let start: number | null = null
    function tickDeriva(ts: number) {
      if (!start) start = ts
      const t = (ts - start) / 1000
      const dx = Math.sin((t / effect.velocidade) * Math.PI * 2 + ox) * amp
      const dy = Math.cos((t / effect.velocidade) * Math.PI * 2 * 0.7 + oy) * amp * 0.6
      el.setAttribute('transform', base + ' translate(' + dx + ',' + dy + ')')
      activeAnimations.set(id, requestAnimationFrame(tickDeriva))
    }
    activeAnimations.set(id, requestAnimationFrame(tickDeriva))
  }

  else if (effect.tipo === 'aparecer') {
    let start: number | null = null
    const phaseOffsetA = Math.random() * Math.PI * 2
    function tickAparecer(ts: number) {
      if (!start) start = ts
      const phase = ((ts - start) / 1000 / effect.velocidade) * Math.PI * 2 + phaseOffsetA
      el.style.opacity = String((Math.sin(phase) + 1) / 2)
      activeAnimations.set(id, requestAnimationFrame(tickAparecer))
    }
    activeAnimations.set(id, requestAnimationFrame(tickAparecer))
  }

  else if (effect.tipo === 'blur') {
    let start: number | null = null
    const minB = amp * 0.3
    const maxB = amp
    const phaseOffsetB = Math.random() * Math.PI * 2
    function tickBlur(ts: number) {
      if (!start) start = ts
      const phase = ((ts - start) / 1000 / effect.velocidade) * Math.PI * 2 + phaseOffsetB
      const val = minB + ((Math.sin(phase) + 1) / 2) * (maxB - minB)
      el.style.filter = 'blur(' + val.toFixed(1) + 'px)'
      activeAnimations.set(id, requestAnimationFrame(tickBlur))
    }
    activeAnimations.set(id, requestAnimationFrame(tickBlur))
  }

  else if (effect.tipo === 'organica') {
    const svgEl = el.closest('svg')
    if (!svgEl) return
    const filterId = 'prev-org-' + id
    let defs = svgEl.querySelector('defs')
    if (!defs) {
      defs = document.createElementNS('http://www.w3.org/2000/svg', 'defs')
      svgEl.prepend(defs)
    }
    const old = defs.querySelector('#' + filterId)
    if (old) old.remove()
    const seed = Math.floor(Math.random() * 100)
    const freq = 0.015 + amp * 0.001
    const filterEl = document.createElementNS('http://www.w3.org/2000/svg', 'filter')
    filterEl.setAttribute('id', filterId)
    filterEl.setAttribute('x', '-20%')
    filterEl.setAttribute('y', '-20%')
    filterEl.setAttribute('width', '140%')
    filterEl.setAttribute('height', '140%')
    filterEl.innerHTML = '<feTurbulence type="fractalNoise" baseFrequency="' + freq.toFixed(4) + '" numOctaves="3" seed="' + seed + '" result="n"><animate attributeName="baseFrequency" values="' + freq.toFixed(4) + ';' + (freq+0.008).toFixed(4) + ';' + freq.toFixed(4) + '" dur="' + effect.velocidade + 's" repeatCount="indefinite"/></feTurbulence><feDisplacementMap in="SourceGraphic" in2="n" scale="' + amp + '"/>'
    defs.appendChild(filterEl)
    el.setAttribute('filter', 'url(#' + filterId + ')')
  }
}

function clearPreview(el: SVGElement) {
  stopAnimation(el.id)
  el.style.filter = ''
  el.style.opacity = ''
  const orig = el.getAttribute('data-original-transform')
  if (orig !== null) {
    if (orig) el.setAttribute('transform', orig)
    else el.removeAttribute('transform')
  }
  el.removeAttribute('filter')
}

export default function App() {
  const [svgContent, setSvgContent] = useState<string>('')
  const [elementos, setElementos] = useState<ElementConfig[]>([])
  const [selecionado, setSelecionado] = useState<string | null>(null)
  const fileRef = useRef<HTMLInputElement>(null)
  const canvasRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!canvasRef.current) return
    elementos.forEach(elem => {
      const el = canvasRef.current!.querySelector('#' + elem.id) as SVGElement | null
      if (!el) return
      if (elem.animacao) {
        applyPreview(el, elem.animacao)
      } else {
        clearPreview(el)
      }
    })
  }, [elementos])

  useEffect(() => {
    stopAllAnimations()
  }, [svgContent])

  function handleFile(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0]
    if (!file) return
    const reader = new FileReader()
    reader.onload = (ev) => {
      const text = ev.target?.result as string
      setSvgContent(text)
      parseLayers(text)
    }
    reader.readAsText(file)
  }

  function parseLayers(svg: string) {
    const parser = new DOMParser()
    const doc = parser.parseFromString(svg, 'image/svg+xml')
    const all = doc.querySelectorAll('[id]')
    const elems: ElementConfig[] = []
    all.forEach((el) => {
      const id = el.getAttribute('id') || ''
      const tag = el.tagName
      if (id && !['defs', 'style', 'metadata'].includes(tag)) {
        elems.push({ id, tag })
      }
    })
    setElementos(elems)
    setSelecionado(null)
  }

  function setEfeito(id: string, tipo: string) {
    setElementos(prev => prev.map(el =>
      el.id === id ? { ...el, animacao: { tipo, intensidade: 5, velocidade: 5 } } : el
    ))
  }

  function removeEfeito(id: string) {
    setElementos(prev => prev.map(el =>
      el.id === id ? { ...el, animacao: undefined } : el
    ))
  }

  function setParam(id: string, param: 'intensidade' | 'velocidade', value: number) {
    setElementos(prev => prev.map(el =>
      el.id === id && el.animacao ? { ...el, animacao: { ...el.animacao, [param]: value } } : el
    ))
  }

  function exportarConfig() {
    const config = {
      nome: 'Ilustração EduSVG',
      svg_path: 'ilustracao.svg',
      elementos: elementos.filter(e => e.animacao).map(e => ({ id: e.id, animacao: e.animacao }))
    }
    const blob = new Blob([JSON.stringify(config, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'config.json'
    a.click()
  }

  const elemSelecionado = elementos.find(e => e.id === selecionado)

  return (
    <div className="editor-root">
      <header className="editor-header">
        <div className="header-logo">
          <span className="logo-edu">Edu</span><span className="logo-svg">SVG</span>
        </div>
        <div className="header-actions">
          <button className="btn-ghost" onClick={() => fileRef.current?.click()}>Importar SVG</button>
          <button className="btn-primary" onClick={exportarConfig} disabled={!svgContent}>Exportar Config</button>
          <input ref={fileRef} type="file" accept=".svg" onChange={handleFile} style={{display:'none'}} />
        </div>
      </header>
      <main className="editor-main">
        <aside className="panel panel-layers">
          <div className="panel-title">Camadas</div>
          {elementos.length === 0 && <div className="empty-state">Importe um SVG para ver as camadas</div>}
          {elementos.map(el => (
            <div key={el.id} className={'layer-item' + (selecionado === el.id ? ' selected' : '')} onClick={() => setSelecionado(el.id)}>
              <div className="layer-info">
                <span className="layer-tag">{el.tag}</span>
                <span className="layer-id">{el.id}</span>
              </div>
              {el.animacao && <span className="layer-badge" style={{background: EFEITO_COR[el.animacao.tipo]}}>{el.animacao.tipo}</span>}
            </div>
          ))}
        </aside>
        <section className="panel panel-canvas">
          {!svgContent && (
            <div className="canvas-empty" onClick={() => fileRef.current?.click()}>
              <div className="canvas-empty-icon">⊕</div>
              <div>Clique para importar um SVG</div>
            </div>
          )}
          {svgContent && <div ref={canvasRef} className="svg-preview" dangerouslySetInnerHTML={{ __html: svgContent }} />}
        </section>
        <aside className="panel panel-props">
          <div className="panel-title">Efeitos</div>
          {!elemSelecionado && <div className="empty-state">Selecione uma camada</div>}
          {elemSelecionado && (
            <div className="props-content">
              <div className="props-id">{elemSelecionado.id}</div>
              <div className="props-tag">{elemSelecionado.tag}</div>
              <div className="efeitos-grid">
                {EFEITOS.map(tipo => (
                  <button key={tipo}
                    className={'efeito-chip' + (elemSelecionado.animacao?.tipo === tipo ? ' active' : '')}
                    style={elemSelecionado.animacao?.tipo === tipo ? {background: EFEITO_COR[tipo], color: '#000'} : {}}
                    onClick={() => elemSelecionado.animacao?.tipo === tipo ? removeEfeito(elemSelecionado.id) : setEfeito(elemSelecionado.id, tipo)}
                  >{tipo}</button>
                ))}
              </div>
              {elemSelecionado.animacao && (
                <div className="sliders">
                  <div className="slider-row">
                    <label>Intensidade</label>
                    <input type="range" min="1" max="20" value={elemSelecionado.animacao.intensidade}
                      onChange={e => setParam(elemSelecionado.id, 'intensidade', Number(e.target.value))} />
                    <span>{elemSelecionado.animacao.intensidade}</span>
                  </div>
                  <div className="slider-row">
                    <label>Velocidade</label>
                    <input type="range" min="1" max="20" value={elemSelecionado.animacao.velocidade}
                      onChange={e => setParam(elemSelecionado.id, 'velocidade', Number(e.target.value))} />
                    <span>{elemSelecionado.animacao.velocidade}</span>
                  </div>
                </div>
              )}
            </div>
          )}
        </aside>
      </main>
    </div>
  )
}
