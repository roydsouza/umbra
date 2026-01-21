import { useState, useEffect } from 'react';
import { ReactFlow, Controls, Background, useNodesState, useEdgesState, BackgroundVariant, MarkerType } from '@xyflow/react';
import type { Node, Edge } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { invoke } from '@tauri-apps/api/core';
import { GlassCard } from '../components/ui/GlassCard';

interface CircuitNode {
    fingerprint: string;
    nickname: string;
    country: string;
    role: string;
}

interface CircuitInfo {
    id: string;
    state: string;
    path: CircuitNode[];
    age_seconds: number;
}

// Custom Node Style for Space Theme
const nodeStyle = {
    background: '#0a0a1a',
    color: '#fff',
    border: '1px solid rgba(74,158,255,0.3)',
    borderRadius: '8px',
    padding: '10px',
    boxShadow: '0 0 10px rgba(74,158,255,0.1)',
    fontFamily: 'JetBrains Mono, monospace'
};

export const Circuits = () => {
    const [circuits, setCircuits] = useState<CircuitInfo[]>([]);
    const [nodes, setNodes, onNodesChange] = useNodesState<Node>([]);
    const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>([]);

    const fetchCircuits = async () => {
        try {
            const data = await invoke<CircuitInfo[]>('get_circuits');
            setCircuits(data);

            // Transform data for React Flow
            const newNodes: Node[] = [];
            const newEdges: Edge[] = [];

            data.forEach((circuit, cIndex) => {
                const yPos = cIndex * 150;

                // Client Node
                const clientId = `client-${cIndex}`;
                newNodes.push({
                    id: clientId,
                    position: { x: 0, y: yPos },
                    data: { label: 'You (MissionControl)' },
                    style: { ...nodeStyle, borderColor: '#8b5cf6', boxShadow: '0 0 15px rgba(139, 92, 246, 0.2)' }
                });

                let prevId = clientId;

                circuit.path.forEach((hop, hIndex) => {
                    const hopId = `${circuit.id}-${hIndex}`;
                    const xPos = (hIndex + 1) * 250;

                    newNodes.push({
                        id: hopId,
                        position: { x: xPos, y: yPos },
                        data: {
                            label: (
                                <div className="text-center">
                                    <div className="font-bold text-accent-blue">{hop.nickname}</div>
                                    <div className="text-xs text-text-muted">{hop.country} | {hop.role}</div>
                                    <div className="text-[10px] text-text-muted mt-1 opacity-50">{hop.fingerprint.substring(0, 8)}...</div>
                                </div>
                            )
                        },
                        style: nodeStyle
                    });

                    newEdges.push({
                        id: `e-${prevId}-${hopId}`,
                        source: prevId,
                        target: hopId,
                        animated: true,
                        style: { stroke: circuit.state === 'Ready' ? '#22c55e' : '#eab308' },
                        markerEnd: { type: MarkerType.ArrowClosed, color: circuit.state === 'Ready' ? '#22c55e' : '#eab308' },
                    });

                    prevId = hopId;
                });

                // Destination Node (Generic)
                const destId = `dest-${cIndex}`;
                newNodes.push({
                    id: destId,
                    position: { x: (circuit.path.length + 1) * 250, y: yPos },
                    data: { label: 'Internet' },
                    style: { ...nodeStyle, borderColor: '#fff', background: 'rgba(255,255,255,0.05)' }
                });

                newEdges.push({
                    id: `e-${prevId}-${destId}`,
                    source: prevId,
                    target: destId,
                    animated: true,
                    style: { stroke: '#4a9eff', strokeDasharray: '5,5' },
                });
            });

            setNodes(newNodes);
            setEdges(newEdges);

        } catch (err) {
            console.error("Failed to fetch circuits:", err);
        }
    };

    useEffect(() => {
        fetchCircuits();
        const interval = setInterval(fetchCircuits, 5000); // Poll every 5s
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="h-[calc(100vh-100px)] flex flex-col gap-4">
            <div className="flex justify-between items-center">
                <h1 className="text-2xl font-bold flex items-center gap-2">
                    <span className="text-accent-blue">Active Circuits</span>
                    <span className="text-sm font-mono text-text-muted bg-white/5 px-2 py-0.5 rounded">
                        {circuits.length} CONNECTED
                    </span>
                </h1>
                <button
                    onClick={fetchCircuits}
                    className="px-4 py-2 bg-white/5 hover:bg-white/10 rounded-lg text-sm border border-glass-border transition-colors"
                >
                    Refresh
                </button>
            </div>

            <GlassCard className="flex-1 w-full overflow-hidden" hoverEffect={false}>
                <ReactFlow
                    nodes={nodes}
                    edges={edges}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    fitView
                    className="bg-transparent"
                >
                    <Background color="#4a9eff" gap={20} size={1} variant={BackgroundVariant.Dots} className="opacity-10" />
                    <Controls className="bg-bg-tertiary border border-glass-border text-white fill-white" />
                </ReactFlow>
            </GlassCard>
        </div>
    );
};
