const vscode = require('vscode');

function activate(context) {
    const run = vscode.commands.registerCommand('holy.runFile', () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) return;

        const file = editor.document.fileName;

        let terminal = vscode.window.terminals.find(t => t.name === 'Holy');
        if (!terminal) {
            terminal = vscode.window.createTerminal('Holy');
        }

        terminal.show(true);
        terminal.sendText(`holy --color "${file}"`);
    });

    context.subscriptions.push(run);
}

module.exports = { activate };
