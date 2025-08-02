using System;
using System.Collections.Generic;
using System.IO;

namespace BalancedExpressionsChecker
{

    class Program
    {
        private static readonly Dictionary<char, char> OpeningBrackets = new Dictionary<char, char>
        {
            { '(', ')' },
            { '[', ']' },
            { '{', '}' }
        };

        private static readonly HashSet<char> ClosingBrackets = new HashSet<char> { ')', ']', '}' };

        static void Main(string[] args)
        {
            Console.WriteLine("----- Ejercicio 2 -----\n");

           
            string fileName = "expresiones.txt";

            try
            {
               
                ProcessFile(fileName);
            }
            catch (FileNotFoundException)
            {
                Console.WriteLine($"Error: El archivo '{fileName}' no fue encontrado.");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error inesperado: {ex.Message}");
            }

            Console.WriteLine("\nPresione cualquier tecla para salir...");
            Console.ReadKey();
        }


        private static void ProcessFile(string fileName)
        {

            string[] lines = File.ReadAllLines(fileName);
            
            if (lines.Length == 0)
            {
                Console.WriteLine("El archivo está vacío.");
                return;
            }

            // Procesar cada línea del archivo
            for (int i = 0; i < lines.Length; i++)
            {
                string expression = lines[i].Trim();
                
                if (string.IsNullOrEmpty(expression))
                {
                    Console.WriteLine($"Línea {i + 1}: [VACÍA] - Omitida\n");
                    continue;
                }

                Console.WriteLine($"Línea {i + 1}: {expression}");
                Console.WriteLine(new string('-', 50));
                
                bool isBalanced = CheckBalance(expression);
                
                Console.WriteLine($"Resultado: {(isBalanced ? "BALANCEADA" : "NO BALANCEADA")}");
                Console.WriteLine();
            }
        }


        private static bool CheckBalance(string expression)
        {
            Stack<char> stack = new Stack<char>();
            int position = 0;

            Console.WriteLine("Procesando carácter por carácter:");
            Console.WriteLine(new string('-', 45));

            foreach (char c in expression)
            {
                position++;
                
                // Si es un símbolo de apertura
                if (OpeningBrackets.ContainsKey(c))
                {
                    stack.Push(c);
                    Console.WriteLine($"{position,2}  | '{c}'  | PUSH      | [{GetStackContent(stack)}]");
                }
                // Si es un símbolo de cierre
                else if (ClosingBrackets.Contains(c))
                {
                    if (stack.Count == 0)
                    {
                        Console.WriteLine($"{position,2}  | '{c}'  | POP ERROR | [VACÍA] - No hay apertura correspondiente");
                        return false;
                    }

                    char lastOpening = stack.Pop();
                    
                    // Verificar si el símbolo de cierre corresponde al último de apertura
                    if (OpeningBrackets[lastOpening] != c)
                    {
                        Console.WriteLine($"{position,2}  | '{c}'  | POP ERROR | Esperaba '{OpeningBrackets[lastOpening]}' pero encontró '{c}'");
                        return false;
                    }
                    
                    Console.WriteLine($"{position,2}  | '{c}'  | POP '{lastOpening}'    | [{GetStackContent(stack)}]");
                }
                // Si no es un símbolo de balance, no se muestra (para mantener la salida limpia)
            }

            // La expresión está balanceada si la pila está vacía al final
            bool isBalanced = stack.Count == 0;
            
            if (!isBalanced)
            {
                Console.WriteLine($"FINAL: Pila no vacía | [{GetStackContent(stack)}] - Símbolos sin cerrar");
            }
            else
            {
                Console.WriteLine("FINAL: Pila vacía | [] - Todos los símbolos están balanceados");
            }

            return isBalanced;
        }

        
        private static string GetStackContent(Stack<char> stack)
        {
            if (stack.Count == 0)
                return "";

            // Convertir la pila a array para mostrar desde el fondo hacia arriba
            char[] stackArray = stack.ToArray();
            Array.Reverse(stackArray);
            return string.Join(", ", stackArray);
        }
    }
}