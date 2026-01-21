//! Icon constants for MCP tools
//! From Microsoft Fluent UI Emoji (MIT License)

use rmcp::model::Icon;

/// Create an Icon from a data URI
fn icon(src: &str) -> Icon {
    Icon {
        src: src.to_string(),
        mime_type: Some("image/png".to_string()),
        sizes: Some(vec!["256x256".to_string()]),
    }
}

/// Waving hand icon for greeting tools
#[must_use]
pub fn waving_hand() -> Vec<Icon> {
    vec![icon("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAEACAYAAABccqhmAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAIS0SURBVHgB7X0HnCVVlfc5t6pe6DQzDAMMIFFExYzKuqsuoK6gi7LqIKAoGDCtrAldV8U2fK4560pSEARhXFSSYXVBl9VdRdcEqKhDzkzo+ELVvd/Nde+tqtc9M90zzVBnfjWV3qtX73Wd//mfcM8FqKWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWpaaINRSyyLLmouva2yajE5IW8sPJ3F7hBHSZYBTJCL3Y0bv40/hLRiTv/RY9w9X/8OKjVDLNpMaAGpZVDn6U1ctnyHt7/WHVj2ZNUYBkhYAIfrJQ0BUi9gnwCjfvhkYuzYi5BpC0quu/NXwdTCOFGpZFKkBoJZFled9+KqzZlurXsmtP7DGMEDc5E+dUni1qEcQkak38DUC08DAgCDcyncvbyC5uM1a/7X2GMyglgWTGgBqWTQ5cfyq1sbm6P2zzZVDaXMEaNLmDKAhzzGh4HJtMIBJ5QfQxl5uMwkC4rwgDRER7ADOwRjP+s4Lh26DWrZaagCoZdHkxI9et9tkmtzZaYxBn1t/yq0/i2N5jmkSIO2+ZAJG+QUQEKpBgHIRAcofBAAM+BN1gH7oMI/PCOjMgNxX8QFZ0y+BwFTv6eHBqm5XAgHhAEMiMsUjBf/8vRPgi3mvsM2Xl71l3Skd2v9Mj39eSphOE5r6ACIBQMYDYvjGt089fA08CKQGgFq2u7z0/b98LKXRed2s+eiUNbhbEPEFZWyA2rZfYlB/pDME3FITfNPlb9zlM5vzOWvWXNxo7rLPDTwWsF9PXDtC6QYw4QI0G4YFZEmW7X/Jux95MzwIpGYAtWx3Of+0J/y6PzZ9yFA0fWYTpyGmXYgFE8gyywagn+oqwh5QwQZS+sm//+w9L9qcz1m79phejPQTEeXOhmAAVC2GAYBiAVEWRy+BB4nUAFDLkpC1b/nr2fPGDz65mXRe18bpfsI6HAj6EgRECS/2uXL2MwkEjC+03yM07Z/73I/fcfDmfM40nf0qDz1uECCAZcqamqGyBRYAcmEOKBRTf1Di+4dMAPxMgBCU4fmXDfqtCKY/J+gEAk1ln1kofdiRb5x7cNCDUWoAqGWbSAaCGdAAAAAElFTkSuQmCC")]
}

/// Question mark icon for query tools
#[must_use]
pub fn question() -> Vec<Icon> {
    // Placeholder - using waving hand icon
    waving_hand()
}

/// Speech bubble icon for communication tools
#[must_use]
pub fn speech() -> Vec<Icon> {
    // Placeholder - using waving hand icon
    waving_hand()
}

/// Abacus icon for calculation tools
#[must_use]
pub fn abacus() -> Vec<Icon> {
    // Placeholder - using waving hand icon
    waving_hand()
}

/// Sun behind cloud icon for weather tools
#[must_use]
pub fn sun_behind_cloud() -> Vec<Icon> {
    vec![icon("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAEACAYAAABccqhmAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAIS0SURBVHgB7X0HnCVVlfc5t6pe6DQzDAMMIFFExYzKuqsuoK6gi7LqIKAoGDCtrAldV8U2fK4560pSEARhXFSSYXVBl9VdRdcEqKhDzkzo+ELVvd/Nde+tqtc9M90zzVBnfjWV3qtX73Wd//mfcM8FqKWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWpaaINRSyyLLmouva2yajE5IW8sPJ3F7hBHSZYBTJCL3Y0bv40/hLRiTv/RY9w9X/8OKjVDLNpMaAGpZVDn6U1ctnyHt7/WHVj2ZNUYBkhYAIfrJQ0BUi9gnwCjfvhkYuzYi5BpC0quu/NXwdTCOFGpZFKkBoJZFled9+KqzZlurXsmtP7DGMEDc5E+dUni1qEcQkak38DUC08DAgCDcyncvbyC5uM1a/7X2GMyglgWTGgBqWTQ5cfyq1sbm6P2zzZVDaXMEaNLmDKAhzzGh4HJtMIBJ5QfQxl5uMwkC4rwgDRER7ADOwRjP+s4Lh26DWrZaagCoZdHkxI9et9tkmtzZaYxBn1t/yq0/i2N5jmkSIO2+ZAJG+QUQEKpBgHIRAcofBAAM+BN1gH7oMI/PCOjMgNxX8QFZ0y+BwFTv6eHBqm5XAgHhAEMiMsUjBf/8vRPgi3mvsM2Xl71l3Skd2v9Mj39eSphOE5r6ACIBQMYDYvjGt089fA08CKQGgFq2u7z0/b98LKXRed2s+eiUNbhbEPEFZWyA2rZfYlB/pDME3FITfNPlb9zlM5vzOWvWXNxo7rLPDTwWsF9PXDtC6QYw4QI0G4YFZEmW7X/Jux95MzwIpGYAtWx3Of+0J/y6PzZ9yFA0fWYTpyGmXYgFE8gyywagn+oqwh5QwQZS+sm//+w9L9qcz1m79phejPQTEeXOhmAAVC2GAYBiAVEWRy+BB4nUAFDLkpC1b/nr2fPGDz65mXRe18bpfsI6HAj6EgRECS/2uXL2MwkEjC+03yM07Z/73I/fcfDmfM40nf0qDz1uECCAZcqamqGyBRYAcmEOKBRTf1Di+4dMAPxMgBCU4fmXDfqtCKY/J+gEAk1ln1kofdiRb5x7cNCDUWoAqGWbSAaCGdAAAAAElFTkSuQmCC")]
}

/// Robot icon for automation tools
#[must_use]
pub fn robot() -> Vec<Icon> {
    vec![icon("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAEACAYAAABccqhmAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAAXNSR0IArs4c6QAAAARnQU1BAACxjwv8YQUAAIS0SURBVHgB7X0HnCVVlfc5t6pe6DQzDAMMIFFExYzKuqsuoK6gi7LqIKAoGDCtrAldV8U2fK4560pSEARhXFSSYXVBl9VdRdcEqKhDzkzo+ELVvd/Nde+tqtc9M90zzVBnfjWV3qtX73Wd//mfcM8FqKWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWmqppZZaaqmlllpqqaWWWpaaINRSyyLLmouva2yajE5IW8sPJ3F7hBHSZYBTJCL3Y0bv40/hLRiTv/RY9w9X/8OKjVDLNpMaAGpZVDn6U1ctnyHt7/WHVj2ZNUYBkhYAIfrJQ0BUi9gnwCjfvhkYuzYi5BpC0quu/NXwdTCOFGpZFKkBoJZFled9+KqzZlurXsmtP7DGMEDc5E+dUni1qEcQkak38DUC08DAgCDcyncvbyC5uM1a/7X2GMyglgWTGgBqWTQ5cfyq1sbm6P2zzZVDaXMEaNLmDKAhzzGh4HJtMIBJ5QfQxl5uMwkC4rwgDRER7ADOwRjP+s4Lh26DWrZaagCoZdHkxI9et9tkmtzZaYxBn1t/yq0/i2N5jmkSIO2+ZAJG+QUQEKpBgHIRAcofBAAM+BN1gH7oMI/PCOjMgNxX8QFZ0y+BwFTv6eHBqm5XAgHhAEMiMsUjBf/8vRPgi3mvsM2Xl71l3Skd2v9Mj39eSphOE5r6ACIBQMYDYvjGt089fA08CKQGgFq2u7z0/b98LKXRed2s+eiUNbhbEPEFZWyA2rZfYlB/pDME3FITfNPlb9zlM5vzOWvWXNxo7rLPDTwWsF9PXDtC6QYw4QI0G4YFZEmW7X/Jux95MzwIpGYAtWx3Of+0J/y6PzZ9yFA0fWYTpyGmXYgFE8gyywagn+oqwh5QwQZS+sm//+w9L9qcz1m79phejPQTEeXOhmAAVC2GAYBiAVEWRy+BB4nUAFDLkpC1b/nr2fPGDz65mXRe18bpfsI6HAj6EgRECS/2uXL2MwkEjC+03yM07Z/73I/fcfDmfM40nf0qDz1uECCAZcqamqGyBRYAcmEOKBRTf1Di+4dMAPxMgBCU4fmXDfqtCKY/J+gEAk1ln1kofdiRb5x7cNCDUWoAqGWbSAaCGdAAAAAElFTkSuQmCC")]
}

/// Hourglass icon for long-running tasks
#[must_use]
pub fn hourglass() -> Vec<Icon> {
    // Placeholder - using waving hand icon
    waving_hand()
}

/// Package icon for loading/installation tools
#[must_use]
pub fn package() -> Vec<Icon> {
    // Placeholder - using waving hand icon
    waving_hand()
}

/// Thought balloon icon for AI/thinking tools
#[must_use]
pub fn thought_balloon() -> Vec<Icon> {
    // Placeholder - using waving hand icon
    waving_hand()
}
